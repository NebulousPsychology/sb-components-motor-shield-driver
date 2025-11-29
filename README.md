# sb-components-motor-shield-driver
Rust driver for board components of
<https://shop.sb-components.co.uk/collections/hats-for-raspberry-pi/products/motorshield-for-raspberry-pi>

<https://github.com/sbcshop/MotorShield>


<https://github.com/sbcshop/MotorShield/blob/master/PiMotor.py>
```python
TRIG = 29
ECHO = 31

GPIO.setup(TRIG,GPIO.OUT)
GPIO.setup(ECHO,GPIO.IN)
```

```python
count=0
while True:
    i=0
    avgDistance=0
    for i in range(5):
        # five times, hold trigger low and wait .1 between 
        GPIO.output(TRIG, False)
        time.sleep(0.1)
    
        # ping the trigger high for 0.00_00_1
        GPIO.output(TRIG, True) 
        time.sleep(0.00001)
        GPIO.output(TRIG, False)
        
        # wait for the echo, refreshing the pulse_start
        while GPIO.input(ECHO)==0:
            pulse_start = time.time()
    
        while GPIO.input(ECHO)==1:
            pulse_end = time.time()
            pulse_duration = pulse_end - pulse_start # get the timespan of the echoed pulse
    
            # calculate the distance
            distance = (pulse_duration * 34300)/2
            distance = round(distance,2)

            avgDistance=avgDistance+distance
            # perform a running average
            avgDistance=avgDistance/5
            print(avgDistance)
    
        if avgDistance < 20:
            count=count+1
            stop()
            time.sleep(1)
            back()
            time.sleep(2)
            if (count%4 ==1):
                right()
    
            else:
                forward()
```
