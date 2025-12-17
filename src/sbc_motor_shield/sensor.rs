#![allow(unused)]

use embedded_hal::{
    digital::{self, Error},
    pwm,
};

pub trait ISensor<T> {
    fn read(&mut self) -> Option<T>;
}

pub mod infrared {
    use super::*;
    pub struct SensorIR<TPin>
    where
        TPin: digital::InputPin,
    {
        pin: TPin,
    }

    impl<TEchoPin> SensorIR<TEchoPin>
    where
        TEchoPin: digital::InputPin,
    {
        pub fn new(op: TEchoPin) -> Self {
            SensorIR { pin: op }
        }
    }

    impl<TPin> ISensor<bool> for SensorIR<TPin>
    where
        TPin: digital::InputPin,
    {
        fn read(&mut self) -> Option<bool> {
            self.pin.is_high().ok()
        }
    }
}

/// # Manage the HC-SR04 Ultrasonic Sensor
pub mod ultrasonic {

    use super::*;

    fn sample<TReceiverPin: digital::InputPin, TTriggerPin: digital::OutputPin>(
        a: TReceiverPin,
        b: TTriggerPin,
    ) {
        // the way Hcsr04 declares generics `..., TEMP=NoTemperatureCompensation>` could be convenient
        let s: hcsr04::Hcsr04<TTriggerPin, TReceiverPin, _, NoTemperatureCompensation> =
            hcsr04::Hcsr04::builder().trig(b).echo(a).delay(100).build();
    }

    /// # Ultrasonic Sensor Driver
    ///
    /// Operates the HC-SR04 ultrasonic sensor as used on the SBC Motor Shield
    /// <https://lastminuteengineers.com/arduino-sr04-ultrasonic-sensor-tutorial>
    /// The HC-SR04 can measure distances from 2 cm to 400 cm (about 0.8 inches to 157 inches) with an accuracy of about 3 mm.
    #[deprecated(note = "switch to hcsr04 crate")]
    pub struct Sonar<TReceiverPin, TTriggerPin>
    where
        TReceiverPin: digital::InputPin,
        TTriggerPin: digital::OutputPin,
    {
        trigger: TTriggerPin,
        echo_receiver: TReceiverPin,
        last_read_mm: Option<u32>,
        boundary: u32,
    }

    impl<TReceiverPin, TTriggerPin> Sonar<TReceiverPin, TTriggerPin>
    where
        TReceiverPin: digital::InputPin,
        TTriggerPin: digital::OutputPin,
    {
        pub fn new(trigger: TTriggerPin, echo_receiver: TReceiverPin) -> Self {
            Sonar {
                trigger,
                echo_receiver,
                last_read_mm: None,
                boundary: 0,
            }
        }

        ///
        /// <https://github.com/sbcshop/MotorShield/blob/master/PiMotor.py> `def sonicCheck`
        ///
        /// <https://github.com/sbcshop/MotorShield/blob/master/ultra.py>
        ///
        /// - five times, hold trigger low and wait .1 between
        /// - ping the trigger high for 0.00_00_1
        /// wait for the echo, refreshing the pulse_start
        /// while the echo is high, get the timespan of the echoed pulse
        ///
        pub fn ping(&mut self, delay_source: &mut cortex_m::delay::Delay) -> () {
            // TODO: revisit with async
            const SETTLING_DELAY_MS: u32 = 333;
            const TRIGGER_SIGNAL_US: u32 = 10;
            delay_source.delay_ms(SETTLING_DELAY_MS);
            self.trigger.set_high().unwrap();
            delay_source.delay_us(TRIGGER_SIGNAL_US);
            self.trigger.set_low().unwrap();

            // "As soon as these sound waves are sent out, the Echo pin goes HIGH, and
            // the sensor starts waiting for the echo to return."
            // wait until the echo is no longer low to start the duration measurement
            let mut start = cortex_m::peripheral::SYST::get_current();
            while self.echo_receiver.is_low().unwrap() {
                start = cortex_m::peripheral::SYST::get_current();
            }

            // "If the sound waves hit an object and bounce back to the sensor,
            // the Echo pin goes LOW as soon as it detects this returning echo."
            // ... or until timeout at 38ms
            let mut stop = cortex_m::peripheral::SYST::get_current();
            while self.echo_receiver.is_high().unwrap() {
                stop = cortex_m::peripheral::SYST::get_current();
            }

            // The echo pin is high while the pulse is in flight, until the echo returns.
            // Measure how long the echo pin was high
            let dt = fugit::MicrosDurationU32::from_ticks(stop - start);

            // FIXME: check that we get time and not ticks
            self.calculate_distance(dt);
        }

        fn calculate_distance(&mut self, t_waiting_for_echo: fugit::MicrosDurationU32) -> u32 {
            // speed of sound in air 343 m/s; half speed of sound 171.5 m/s

            // 0.000_171_5 m/us
            const HALF_SOS_CM_PER_US: f32 = 0.017_15; // 0.017_15_ cm/us
            const HALF_SOS_MM_PER_US: f32 = 0.171_5; // 0.171_5__ mm/us
                                                     // 171.5 um/us
                                                     // 171_500 nm/us

            // K_speed * round trip time of flight = round trip distance
            let measure: f32 = HALF_SOS_MM_PER_US * t_waiting_for_echo.to_micros() as f32;
            self.last_read_mm = Some(measure as u32);
            return self.last_read_mm.unwrap();
        }

        /// ## Returns `true` if the last measured distance is less than the boundary
        fn boundary_check(&mut self, b: u32) -> bool {
            self.boundary = b;
            return match self.last_read_mm {
                Some(dist_mm) => dist_mm < self.boundary,
                None => false,
            };
        }
    }

    impl<TReceiverPin, TTriggerPin> ISensor<u32> for Sonar<TReceiverPin, TTriggerPin>
    where
        TReceiverPin: digital::InputPin,
        TTriggerPin: digital::OutputPin,
    {
        fn read(&mut self) -> Option<u32> {
            self.last_read_mm
        }
    }
}
