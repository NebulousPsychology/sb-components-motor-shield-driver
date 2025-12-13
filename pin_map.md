# pins

Rpi4 BCM is the intended mapping for the sb components shield.  
Pico BCM names are assignments used by a geeekpi breakout hat board for pico.

| Pico BCM |  RPI BCM   |SBC purpose|`L`|`R`|SBC purpose|   RPI BCM  | Pico BCM |
|----------|------------|----------:|--:|:--|:----------|-----------:|---------:|
|    -     |     -      |  **3V3**  |  1| 2 |   **5V**  |     -      |     -    |
|   20     |gp2/i2c1sda | _?i2c0da_ |  3| 4 |   **5V**  |     -      |     -    |
|   21     |gp3/i2c1scl | _?i2c0cl_ |  5| 6 |  **GND**  |     -      |     -    |
|    6     |gp4         | `IR1-iu`  |  7| 8 | _?UARTTX_ |gp14/uart0tx|   0/u0tx |
|    -     |     -      |  **GND**  |  9|10 | _?UARTRX_ |gp15/uart0rx|   1/u0rx |
|    7     |   gp17     |`M1EN(pwm)`| 11|12 | `IR2-Iu`  |gp18/pwm0   |      28  |
|    8     |   gp27     |`M1BK`     | 13|14 |  **GND**  |     -      |    -     |
|    9     |   gp22     |`M1FW`     | 15|16 | `M2FW`    |      gp23  |      27  |
|    -     |     -      |  **3v3**  | 17|18 | `M2BK`    |      gp24  |      26  |
|    3     |   gp10     |`M3EN(PWM)`| 19|20 |  **GND**  |     -      |    -     |
|    4     |   gp9      |`M3FW`     | 21|22 |`M2EN(PWM)`|      gp25  | 22       |
|    2     |   gp11     |`M3BK`     | 23|24 |    `M4FW` | gp8/spi    |  5       |
|    -     |     -      |  **GND**  | 25|26 |    `M4BK` | gp7        | 19       |
|~~X dnc~~ |~~gp0 i2cXda~~|  _?nc_  | 27|28 |  _nc?_    | ~~gp1 i2cXcl~~ |      ~~X~~    |
|    0>>10 |     gp5    |`US1TR-o`  | 29|30 |  **GND**  |   -        |   -      |
|    11    |     gp6    |`US1EN-iu` | 31|32 |`M4EN(PWM)`|  gp12/pwm0 | 18       |
|    12    | gp13/pwm1  |  `LED_B`  | 33|34 |  **GND**  |     -      |   -      |
|    13    | gp19/pwm1  |  `LED_L`  | 35|36 |  `LED_R`  |      gp16  | 17       |
|    14    |    gp26    |  `LED_F`  | 37|38 | _?us2en-i_|      gp20  | 16       |
|     -    |     -      |  **GND**  | 39|40 | _?us2tr-o_|      gp21  | 15       |

(key: `confirmed`, **Electrical**, _unused by shield_)

the sb components board is coded in python for software pwm motor control by Rpi. It never promised hardware PWM.

- RpiGP12 can be M4en as PWM0
- rpigp18 is unable to pwm0 while used by m4en? fine, it's just IR2-In
- can RpiGP25 be M2en(PWM)?
- rpigp{13,19} wastes pwm1 on leds?

footed[^1]
p5=us1T

[^1]: a citation footnote not a link
