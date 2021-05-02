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
let target_long = 200
let target_alt = 300

let future_alt = altitude - (alt_vel * 2)
let future_pos = longitude + (long_vel * 2)
let future_angle = angle + (ang_vel * 10)

if (future_alt < target_alt) {
  set_thrust(-3000)
}

if (alt_vel > 20) {
  set_thrust(-3000)
}

let corr_angle = func() {
  if (future_pos < target_long - 30){
    return 10
  }

  if (future_pos > target_long + 30){
    return -10
  }

  0
}

if (future_angle < corr_angle()) {
  set_torque(13000)
} else {
  set_torque(-13000)
}

```
