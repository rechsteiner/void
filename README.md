# VOID

A code-driven space program to explore the great void.

## Installation

Void requires the nightly version of Rust:

```
rustup toolchain install nightly
rustup override set nightly
```

## How to run the project

- Run the continous WASM build with `cargo watch -w "src" -s "wasm-pack build"`
- Front-end continous build `npm run serve`

## How to play

Enter code to control the ship

### Example ship code

This will hover the ship and let it glide to your desired longitude and altitude.

```
LET TARGET_LONG = 200
LET TARGET_ALT = 1050

LET FUTURE_ALT = ALTITUDE - (ALT_VEL * 2)
LET FUTURE_POS = LONGITUDE + (LONG_VEL * 2)
LET FUTURE_ANGLE = ANGLE + (ANG_VEL * 10)

IF FUTURE_ALT < TARGET_ALT DO
  SET_THRUST(3000)
END

IF ALT_VEL < -1 DO
  SET_THRUST(0)
END

IF ALT_VEL > 20 DO
  SET_THRUST(500)
END

LET CORR_ANGLE = FUNC DO
  IF FUTURE_POS < TARGET_LONG - 30 DO
    RETURN 10
  END

  IF FUTURE_POS > TARGET_LONG + 30 DO
    RETURN -10
  END

  0
END

IF FUTURE_ANGLE < CORR_ANGLE() DO
  SET_TORQUE(-13000)
ELSE
  SET_TORQUE(13000)
END
```

Here's a program that makes the ship take off and deliver a cargo into orbit, and then return to the surface.

```
LET GET_ANGLE = FUNC DO
  IF (TIME < 3500) DO
    0
  ELSE
    IF (TIME < 20000) DO
      -90
    ELSE
      IF (TIME < 26500) DO
        90
      ELSE
        0
      END
    END
  END
END


LET TARGET_ANGLE = GET_ANGLE()

IF (TIME > 6000) DO
  TARGET_ANGLE = 90
END

LET ROT_STR = 6000

IF (ANGLE > TARGET_ANGLE) DO
  SET_TORQUE(ROT_STR)
END

IF (ANGLE < TARGET_ANGLE) DO
  SET_TORQUE(-ROT_STR)
END

IF (ANG_VEL > 10) DO
  SET_TORQUE(-ROT_STR + 1600)
END

IF (ANG_VEL < 10) DO
  SET_TORQUE(ROT_STR - 1600)
END


IF (TIME > 3000) DO
  SET_THRUST(3200)
END

IF (TIME > 6500) DO
  SET_THRUST(-2400)
END

IF (TIME > 7500) DO
  SET_THRUST(-400)
END

IF (TIME > 8500) DO
  SET_THRUST(-300)
END

IF (TIME > 14000) DO
  SET_THRUST(-100)
END

IF (TIME > 25000) DO
  SET_THRUST(800)
END
```
