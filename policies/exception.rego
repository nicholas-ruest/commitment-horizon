package commitment_horizon.exception
default allow := false
allow if {
  input.role == "operator"
  count(input.reason) >= 8
  input.duration_minutes > 0
  input.duration_minutes <= 120
}
