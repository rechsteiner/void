# VOID

A code-driven space program to explore the great void.

## How to run the project

- Run the continous WASM build with `cargo watch -w "src" -s "wasm-pack build"`
- Front-end continous build `npm run serve`

## How to play

Enter code to control the ship

### Example ship code

This will hover the ship and let it glide to your desired longitude and altitude.

```
LET TARGET_LONG = 200
LET TARGET_ALT = 300

LET FUTURE_ALT = ALTITUDE - (ALT_VEL * 2)
LET FUTURE_POS = LONGITUDE + (LONG_VEL * 2)
LET FUTURE_ANGLE = ANGLE + (ANG_VEL * 10)

IF FUTURE_ALT < TARGET_ALT DO
  SET_THRUST(-3000)
END

IF ALT_VEL > 20 DO
  SET_THRUST(-3000)
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
  SET_TORQUE(13000)
ELSE
  SET_TORQUE(-13000)
END
```
