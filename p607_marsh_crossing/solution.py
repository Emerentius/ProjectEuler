#!/usr/bin/env python3
import math
from scipy.optimize import bisect 

refr_idx = [1.0, 10/9, 10/8, 10/7, 10/6, 10/5]
delta_ys = [100/math.sqrt(2)-50] + [10]*5
total_delta_x_target = 100.0 / math.sqrt(2)

def total_delta_x_error(alpha0):
    global delta_xs # save for time calculations
    sin0 = math.sin(alpha0)
    delta_xs = [delta_x(sin0, refr_idx[i], delta_ys[i]) for i in range(6)]
    return sum(delta_xs) - total_delta_x_target

def delta_x(sin0, ior, dy):
    # delta_y / slope
    return dy / (1/(sin0 / math.sqrt(1 - (sin0/ior)**2) / ior))

alpha = bisect(total_delta_x_error, 0.1, math.pi/2-0.1) 
sin = math.sin(alpha)

velocities = (10/refr_idx[i] for i in range(6))
lengths = ((dx**2 + dy **2) ** 0.5 for dx, dy in zip(delta_xs, delta_ys))
total_time = sum(l/v for l, v in zip(lengths, velocities))

print(f"{total_time:.10f}")
