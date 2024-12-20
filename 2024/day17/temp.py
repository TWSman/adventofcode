import pandas as pd
import numpy as np

a = [
80911,
    169097903119,
169099475983,
169101573135,
169105767439,
670911446031,
670911511567,
670911642639,
670915640335,
670915705871,
670915836943,
1268609530895,
1268611103759,
1268613200911,
1268617395215,
2368121158671,
2368122731535,
2368124828687,
2368129022991,
3467632786447,
3467634359311,
3467636456463,
3467640650767,
3476222721039,
3476224293903,
3476226391055,
3476230585359,
4567144414223,
4567145987087,
4567148084239,
4567152278543,
5068957957135,
5068958022671,
5068958153743,
5068962151439,
5068962216975,
5068962348047,
5666656041999,
5666657614863,
5666659712015,
5666663906319,
6766167669775,
6766169242639,
6766171339791,
6766175534095,
]

# 14 lengths
a14 = [1268609530895,
1268611103759,
1268613200911,
1268617395215,
5068957957135,
5068958022671,
5068958153743,
5068962151439,
5068962216975,
5068962348047,
5666656041999,
5666657614863,
5666659712015,
5666663906319,
           ]


a15 = [
 27656888597519,
 27656890170383,
 27656892267535,
 27656896461839,

 49647121153039,
 49647122725903,
 49647124823055,
 49647129017359,
 
 62841260686351,
 62841262259215,
 62841264356367,
 62841268550671,

105722214169615,
105722215742479,
105722217839631,
105722222033935,
106821725797391,
106821727370255,
106821729467407,
106821733661711,
#99999999999999 is too low
#9999999999999999 is too high
]


#  11001 0010011101011111000001000000000000000000

#  11001 0010011101011111000111000000000000000000
#  11001 0010011101011111001111000000000000000000
#  11001 0010011101011111011111000000000000000000

# 101101 0010011101011111000001000000000000000000
# 101101 0010011101011111000111000000000000000000
# 101101 0010011101011111001111000000000000000000
# 101101 0010011101011111011111000000000000000000

# 111001 0010011101011111000001000000000000000000
# 111001 0010011101011111000111000000000000000000
# 111001 0010011101011111001111000000000000000000
# 111001 0010011101011111011111000000000000000000

#1100000 0010011101011111000001000000000000000000
#1100000 0010011101011111000111000000000000000000
#1100000 0010011101011111001111000000000000000000
#1100000 0010011101011111011111000000000000000000
#1100001 0010011101011111000001000000000000000000
#1100001 0010011101011111000111000000000000000000
#1100001 0010011101011111001111000000000000000000
#1100001 0010011101011111011111000000000000000000

#       10010011101011111000001010011110000001111
#       10010011101011111000001010011110000001111
#   110010010011101011111000001010011110000001111
#   110010010011101011111000111010011110000001111

#  11001 0010011101011111000001 010011110000001111
#  11001 0010011101011111000111 010011110000001111
#  11001 0010011101011111001111 010011110000001111
#  11001 0010011101011111011111 010011110000001111
# 101101 0010011101011111000001 010011110000001111
# 101101 0010011101011111000111 010011110000001111
# 101101 0010011101011111001111 010011110000001111
# 101101 0010011101011111011111 010011110000001111
# 111001 0010011101011111000001 010011110000001111
# 111001 0010011101011111000111 010011110000001111
# 111001 0010011101011111001111 010011110000001111
# 111001 0010011101011111011111 010011110000001111
#1100000 0010011101011111000001 010011110000001111
#1100000 0010011101011111000111 010011110000001111
#1100000 0010011101011111001111 010011110000001111
#1100000 0010011101011111011111 010011110000001111
#1100001 0010011101011111000001 010011110000001111
#1100001 0010011101011111000111 010011110000001111
#1100001 0010011101011111001111 010011110000001111
#1100001 0010011101011111011111 010011110000001111

# 19275f053c0f
# 19275f1d3c0f
# 19275f3d3c0f
# 19275f7d3c0f

# 2d275f053c0f
# 2d275f1d3c0f
# 2d275f3d3c0f
# 2d275f7d3c0f

# 39275f053c0f
# 39275f1d3c0f
# 39275f3d3c0f
# 39275f7d3c0f

# 60275f053c0f
# 60275f1d3c0f
# 60275f3d3c0f
# 60275f7d3c0f

# 61275f053c0f
# 61275f1d3c0f
# 61275f3d3c0f
# 61275f7d3c0f

def main():
    b = np.array(a15)
    for bb in b:
        print(f"{f'{bb:b}':>13}")
    c = (b[1:] - b[0]) / 65536 / 8
    for cc in c:
        print(cc)

    print()
    c = (b[1:] - b[:-1]) / 65536 / 8
    for cc in c:
        print(cc)


if __name__ == "__main__":
    main()
