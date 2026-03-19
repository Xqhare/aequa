After reading the IEEE 754 standart, I was thinking about how to represent decimals.

While doing a little reasarch, I came across this idea:

1.2 + 0.004 = 1.204

1.2 => 12 | 1 (12 * 10^-1)
0.004 => 4 | 3 (4 * 10^-3)

To add, you align the scales by multiplying the value of the smaller scale by 10^(scale_diff):

scale_diff = 3 - 1 = 2
1.2 => (12 * 10^2) | 3 = 1200 | 3

Then add the values together:
1200 | 3 + 4 | 3 = 1204 | 3

## Going Backwards (Stringification)

To convert 1204 | 3 back into "1.204":
1. Take the value as a string: "1204"
2. Insert the decimal point 3 places from the right: "1.204"

If the value is smaller than the scale (e.g., 4 | 3):
1. Pad the string with leading zeros: "0004"
2. Insert the decimal point: "0.004"

## Naming

Moneta (roman for "money") is a strong contender
Aequa, also roman divine personification of equity, fairness, exact exchange and accurate weights and measurements
