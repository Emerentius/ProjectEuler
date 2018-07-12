#!/usr/bin/env python3
import numpy as np
import random

roll_transitions = np.zeros((40, 40))
event_transitions = np.identity(40)

# Notable Fields:
# Community Chest  2  17 33
# Chance           7  22 36
#   next U         12 28 12
#   next R         15 25 05
#   C1             11
#   E3             24
#   H2             39
#   R1             05
# GO               0
# Jail             10
# G2J              30

next_r = {7: 15, 22: 25, 36: 5}
next_u = {7: 12, 22: 28, 36: 12}

# community chest
for cc in [2, 17, 33]:
    event_transitions[cc, cc] = 14/16
    event_transitions[0, cc] = 1/16
    event_transitions[10, cc] = 1/16

for ch in [7, 22, 36]:
    # stay on field
    event_transitions[ch, ch] = 6/16

    # sent away by chance
    r = next_r[ch]
    # Go, Jail, C1, E3, H2, R1, next_R, next_R, next_U, 3 squares back
    for target in [0, 10, 11, 24, 39, 5, r, r, next_u[ch], ch - 3]:
        event_transitions[0, ch] += 1/16


# Go 2 Jail
event_transitions[10, 30] = 1
event_transitions[30, 30] = 0

event_transitions_first2 = event_transitions.copy()

# three doubles
# ignores that it's impossible in the first 2 rolls, shouldn't matter in the long run
for col in range(40):
    prior_possibility = event_transitions[10, col]
    new_possibility = 1 - (1-1/36)*(1-prior_possibility)
    event_transitions[10, col] = new_possibility

    # normalize
    total = sum(event_transitions[:, col])
    event_transitions[:, col] /= total

# 0 and 1..=12
# n_dice = 2 # hardcoded with 2 for loops. Make recursive or cleverer (binomial) for more
n_sides = 6
length = 2 * n_sides + 1
counts = [0]*length
for roll1 in range(1, n_sides+1):
    for roll2 in range(1, n_sides+1):
        counts[roll1+roll2] += 1

advance_probabilities = np.array(counts, dtype = 'float')
advance_probabilities /= sum(counts)

for start_field in range(40):
    for offset, probability in enumerate(advance_probabilities):
        roll_transitions[(start_field + offset) % 40, start_field] = probability

transition_matrix = np.matrix(event_transitions @ roll_transitions)
transition_matrix_first2 = event_transitions_first2 @ roll_transitions

start_vector = np.zeros(40)
start_vector[0] = 1

stationary = (transition_matrix ** 10000 @ start_vector).A
stationary_corrected = (transition_matrix ** 10000 @ transition_matrix_first2 @ transition_matrix_first2 @ start_vector).A
square_probabilities = list(zip(stationary.T.flatten(), range(40)))
square_probabilities.sort(reverse = True)

solution = ''.join(f"{square:02}" for _, square in  square_probabilities[:3])

print(solution)
