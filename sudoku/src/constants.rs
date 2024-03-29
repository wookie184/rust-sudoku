pub const UNIT_INDICES: [[[usize; 8]; 3]; 81] = [
    [
        [36, 72, 9, 45, 18, 54, 27, 63],
        [1, 2, 3, 4, 5, 6, 7, 8],
        [1, 2, 9, 10, 11, 18, 19, 20],
    ],
    [
        [64, 37, 73, 10, 46, 19, 55, 28],
        [0, 2, 3, 4, 5, 6, 7, 8],
        [0, 2, 9, 10, 11, 18, 19, 20],
    ],
    [
        [65, 38, 74, 11, 47, 20, 56, 29],
        [0, 1, 3, 4, 5, 6, 7, 8],
        [0, 1, 9, 10, 11, 18, 19, 20],
    ],
    [
        [66, 39, 75, 12, 48, 21, 57, 30],
        [0, 1, 2, 4, 5, 6, 7, 8],
        [4, 5, 12, 13, 14, 21, 22, 23],
    ],
    [
        [67, 40, 76, 13, 49, 22, 58, 31],
        [0, 1, 2, 3, 5, 6, 7, 8],
        [3, 5, 12, 13, 14, 21, 22, 23],
    ],
    [
        [32, 68, 41, 77, 14, 50, 23, 59],
        [0, 1, 2, 3, 4, 6, 7, 8],
        [3, 4, 12, 13, 14, 21, 22, 23],
    ],
    [
        [33, 69, 42, 78, 15, 51, 24, 60],
        [0, 1, 2, 3, 4, 5, 7, 8],
        [7, 8, 15, 16, 17, 24, 25, 26],
    ],
    [
        [34, 70, 43, 79, 16, 52, 25, 61],
        [0, 1, 2, 3, 4, 5, 6, 8],
        [6, 8, 15, 16, 17, 24, 25, 26],
    ],
    [
        [35, 71, 44, 80, 17, 53, 26, 62],
        [0, 1, 2, 3, 4, 5, 6, 7],
        [6, 7, 15, 16, 17, 24, 25, 26],
    ],
    [
        [0, 36, 72, 45, 18, 54, 27, 63],
        [10, 11, 12, 13, 14, 15, 16, 17],
        [0, 1, 2, 10, 11, 18, 19, 20],
    ],
    [
        [64, 1, 37, 73, 46, 19, 55, 28],
        [9, 11, 12, 13, 14, 15, 16, 17],
        [0, 1, 2, 9, 11, 18, 19, 20],
    ],
    [
        [65, 2, 38, 74, 47, 20, 56, 29],
        [9, 10, 12, 13, 14, 15, 16, 17],
        [0, 1, 2, 9, 10, 18, 19, 20],
    ],
    [
        [66, 3, 39, 75, 48, 21, 57, 30],
        [9, 10, 11, 13, 14, 15, 16, 17],
        [3, 4, 5, 13, 14, 21, 22, 23],
    ],
    [
        [67, 4, 40, 76, 49, 22, 58, 31],
        [9, 10, 11, 12, 14, 15, 16, 17],
        [3, 4, 5, 12, 14, 21, 22, 23],
    ],
    [
        [32, 68, 5, 41, 77, 50, 23, 59],
        [9, 10, 11, 12, 13, 15, 16, 17],
        [3, 4, 5, 12, 13, 21, 22, 23],
    ],
    [
        [33, 69, 6, 42, 78, 51, 24, 60],
        [9, 10, 11, 12, 13, 14, 16, 17],
        [6, 7, 8, 16, 17, 24, 25, 26],
    ],
    [
        [34, 70, 7, 43, 79, 52, 25, 61],
        [9, 10, 11, 12, 13, 14, 15, 17],
        [6, 7, 8, 15, 17, 24, 25, 26],
    ],
    [
        [35, 71, 8, 44, 80, 53, 26, 62],
        [9, 10, 11, 12, 13, 14, 15, 16],
        [6, 7, 8, 15, 16, 24, 25, 26],
    ],
    [
        [0, 36, 72, 9, 45, 54, 27, 63],
        [19, 20, 21, 22, 23, 24, 25, 26],
        [0, 1, 2, 9, 10, 11, 19, 20],
    ],
    [
        [64, 1, 37, 73, 10, 46, 55, 28],
        [18, 20, 21, 22, 23, 24, 25, 26],
        [0, 1, 2, 9, 10, 11, 18, 20],
    ],
    [
        [65, 2, 38, 74, 11, 47, 56, 29],
        [18, 19, 21, 22, 23, 24, 25, 26],
        [0, 1, 2, 9, 10, 11, 18, 19],
    ],
    [
        [66, 3, 39, 75, 12, 48, 57, 30],
        [18, 19, 20, 22, 23, 24, 25, 26],
        [3, 4, 5, 12, 13, 14, 22, 23],
    ],
    [
        [67, 4, 40, 76, 13, 49, 58, 31],
        [18, 19, 20, 21, 23, 24, 25, 26],
        [3, 4, 5, 12, 13, 14, 21, 23],
    ],
    [
        [32, 68, 5, 41, 77, 14, 50, 59],
        [18, 19, 20, 21, 22, 24, 25, 26],
        [3, 4, 5, 12, 13, 14, 21, 22],
    ],
    [
        [33, 69, 6, 42, 78, 15, 51, 60],
        [18, 19, 20, 21, 22, 23, 25, 26],
        [6, 7, 8, 15, 16, 17, 25, 26],
    ],
    [
        [34, 70, 7, 43, 79, 16, 52, 61],
        [18, 19, 20, 21, 22, 23, 24, 26],
        [6, 7, 8, 15, 16, 17, 24, 26],
    ],
    [
        [35, 71, 8, 44, 80, 17, 53, 62],
        [18, 19, 20, 21, 22, 23, 24, 25],
        [6, 7, 8, 15, 16, 17, 24, 25],
    ],
    [
        [0, 36, 72, 9, 45, 18, 54, 63],
        [32, 33, 34, 35, 28, 29, 30, 31],
        [36, 37, 38, 45, 46, 47, 28, 29],
    ],
    [
        [64, 1, 37, 73, 10, 46, 19, 55],
        [32, 33, 34, 35, 27, 29, 30, 31],
        [36, 37, 38, 45, 46, 47, 27, 29],
    ],
    [
        [65, 2, 38, 74, 11, 47, 20, 56],
        [32, 33, 34, 35, 27, 28, 30, 31],
        [36, 37, 38, 45, 46, 47, 27, 28],
    ],
    [
        [66, 3, 39, 75, 12, 48, 21, 57],
        [32, 33, 34, 35, 27, 28, 29, 31],
        [32, 39, 40, 41, 48, 49, 50, 31],
    ],
    [
        [67, 4, 40, 76, 13, 49, 22, 58],
        [32, 33, 34, 35, 27, 28, 29, 30],
        [32, 39, 40, 41, 48, 49, 50, 30],
    ],
    [
        [68, 5, 41, 77, 14, 50, 23, 59],
        [33, 34, 35, 27, 28, 29, 30, 31],
        [39, 40, 41, 48, 49, 50, 30, 31],
    ],
    [
        [69, 6, 42, 78, 15, 51, 24, 60],
        [32, 34, 35, 27, 28, 29, 30, 31],
        [34, 35, 42, 43, 44, 51, 52, 53],
    ],
    [
        [70, 7, 43, 79, 16, 52, 25, 61],
        [32, 33, 35, 27, 28, 29, 30, 31],
        [33, 35, 42, 43, 44, 51, 52, 53],
    ],
    [
        [71, 8, 44, 80, 17, 53, 26, 62],
        [32, 33, 34, 27, 28, 29, 30, 31],
        [33, 34, 42, 43, 44, 51, 52, 53],
    ],
    [
        [0, 72, 9, 45, 18, 54, 27, 63],
        [37, 38, 39, 40, 41, 42, 43, 44],
        [37, 38, 45, 46, 47, 27, 28, 29],
    ],
    [
        [64, 1, 73, 10, 46, 19, 55, 28],
        [36, 38, 39, 40, 41, 42, 43, 44],
        [36, 38, 45, 46, 47, 27, 28, 29],
    ],
    [
        [65, 2, 74, 11, 47, 20, 56, 29],
        [36, 37, 39, 40, 41, 42, 43, 44],
        [36, 37, 45, 46, 47, 27, 28, 29],
    ],
    [
        [66, 3, 75, 12, 48, 21, 57, 30],
        [36, 37, 38, 40, 41, 42, 43, 44],
        [32, 40, 41, 48, 49, 50, 30, 31],
    ],
    [
        [67, 4, 76, 13, 49, 22, 58, 31],
        [36, 37, 38, 39, 41, 42, 43, 44],
        [32, 39, 41, 48, 49, 50, 30, 31],
    ],
    [
        [32, 68, 5, 77, 14, 50, 23, 59],
        [36, 37, 38, 39, 40, 42, 43, 44],
        [32, 39, 40, 48, 49, 50, 30, 31],
    ],
    [
        [33, 69, 6, 78, 15, 51, 24, 60],
        [36, 37, 38, 39, 40, 41, 43, 44],
        [33, 34, 35, 43, 44, 51, 52, 53],
    ],
    [
        [34, 70, 7, 79, 16, 52, 25, 61],
        [36, 37, 38, 39, 40, 41, 42, 44],
        [33, 34, 35, 42, 44, 51, 52, 53],
    ],
    [
        [35, 71, 8, 80, 17, 53, 26, 62],
        [36, 37, 38, 39, 40, 41, 42, 43],
        [33, 34, 35, 42, 43, 51, 52, 53],
    ],
    [
        [0, 36, 72, 9, 18, 54, 27, 63],
        [46, 47, 48, 49, 50, 51, 52, 53],
        [36, 37, 38, 46, 47, 27, 28, 29],
    ],
    [
        [64, 1, 37, 73, 10, 19, 55, 28],
        [45, 47, 48, 49, 50, 51, 52, 53],
        [36, 37, 38, 45, 47, 27, 28, 29],
    ],
    [
        [65, 2, 38, 74, 11, 20, 56, 29],
        [45, 46, 48, 49, 50, 51, 52, 53],
        [36, 37, 38, 45, 46, 27, 28, 29],
    ],
    [
        [66, 3, 39, 75, 12, 21, 57, 30],
        [45, 46, 47, 49, 50, 51, 52, 53],
        [32, 39, 40, 41, 49, 50, 30, 31],
    ],
    [
        [67, 4, 40, 76, 13, 22, 58, 31],
        [45, 46, 47, 48, 50, 51, 52, 53],
        [32, 39, 40, 41, 48, 50, 30, 31],
    ],
    [
        [32, 68, 5, 41, 77, 14, 23, 59],
        [45, 46, 47, 48, 49, 51, 52, 53],
        [32, 39, 40, 41, 48, 49, 30, 31],
    ],
    [
        [33, 69, 6, 42, 78, 15, 24, 60],
        [45, 46, 47, 48, 49, 50, 52, 53],
        [33, 34, 35, 42, 43, 44, 52, 53],
    ],
    [
        [34, 70, 7, 43, 79, 16, 25, 61],
        [45, 46, 47, 48, 49, 50, 51, 53],
        [33, 34, 35, 42, 43, 44, 51, 53],
    ],
    [
        [35, 71, 8, 44, 80, 17, 26, 62],
        [45, 46, 47, 48, 49, 50, 51, 52],
        [33, 34, 35, 42, 43, 44, 51, 52],
    ],
    [
        [0, 36, 72, 9, 45, 18, 27, 63],
        [55, 56, 57, 58, 59, 60, 61, 62],
        [64, 65, 72, 73, 74, 55, 56, 63],
    ],
    [
        [64, 1, 37, 73, 10, 46, 19, 28],
        [54, 56, 57, 58, 59, 60, 61, 62],
        [64, 65, 72, 73, 74, 54, 56, 63],
    ],
    [
        [65, 2, 38, 74, 11, 47, 20, 29],
        [54, 55, 57, 58, 59, 60, 61, 62],
        [64, 65, 72, 73, 74, 54, 55, 63],
    ],
    [
        [66, 3, 39, 75, 12, 48, 21, 30],
        [54, 55, 56, 58, 59, 60, 61, 62],
        [66, 67, 68, 75, 76, 77, 58, 59],
    ],
    [
        [67, 4, 40, 76, 13, 49, 22, 31],
        [54, 55, 56, 57, 59, 60, 61, 62],
        [66, 67, 68, 75, 76, 77, 57, 59],
    ],
    [
        [32, 68, 5, 41, 77, 14, 50, 23],
        [54, 55, 56, 57, 58, 60, 61, 62],
        [66, 67, 68, 75, 76, 77, 57, 58],
    ],
    [
        [33, 69, 6, 42, 78, 15, 51, 24],
        [54, 55, 56, 57, 58, 59, 61, 62],
        [69, 70, 71, 78, 79, 80, 61, 62],
    ],
    [
        [34, 70, 7, 43, 79, 16, 52, 25],
        [54, 55, 56, 57, 58, 59, 60, 62],
        [69, 70, 71, 78, 79, 80, 60, 62],
    ],
    [
        [35, 71, 8, 44, 80, 17, 53, 26],
        [54, 55, 56, 57, 58, 59, 60, 61],
        [69, 70, 71, 78, 79, 80, 60, 61],
    ],
    [
        [0, 36, 72, 9, 45, 18, 54, 27],
        [64, 65, 66, 67, 68, 69, 70, 71],
        [64, 65, 72, 73, 74, 54, 55, 56],
    ],
    [
        [1, 37, 73, 10, 46, 19, 55, 28],
        [65, 66, 67, 68, 69, 70, 71, 63],
        [65, 72, 73, 74, 54, 55, 56, 63],
    ],
    [
        [2, 38, 74, 11, 47, 20, 56, 29],
        [64, 66, 67, 68, 69, 70, 71, 63],
        [64, 72, 73, 74, 54, 55, 56, 63],
    ],
    [
        [3, 39, 75, 12, 48, 21, 57, 30],
        [64, 65, 67, 68, 69, 70, 71, 63],
        [67, 68, 75, 76, 77, 57, 58, 59],
    ],
    [
        [4, 40, 76, 13, 49, 22, 58, 31],
        [64, 65, 66, 68, 69, 70, 71, 63],
        [66, 68, 75, 76, 77, 57, 58, 59],
    ],
    [
        [32, 5, 41, 77, 14, 50, 23, 59],
        [64, 65, 66, 67, 69, 70, 71, 63],
        [66, 67, 75, 76, 77, 57, 58, 59],
    ],
    [
        [33, 6, 42, 78, 15, 51, 24, 60],
        [64, 65, 66, 67, 68, 70, 71, 63],
        [70, 71, 78, 79, 80, 60, 61, 62],
    ],
    [
        [34, 7, 43, 79, 16, 52, 25, 61],
        [64, 65, 66, 67, 68, 69, 71, 63],
        [69, 71, 78, 79, 80, 60, 61, 62],
    ],
    [
        [35, 8, 44, 80, 17, 53, 26, 62],
        [64, 65, 66, 67, 68, 69, 70, 63],
        [69, 70, 78, 79, 80, 60, 61, 62],
    ],
    [
        [0, 36, 9, 45, 18, 54, 27, 63],
        [73, 74, 75, 76, 77, 78, 79, 80],
        [64, 65, 73, 74, 54, 55, 56, 63],
    ],
    [
        [64, 1, 37, 10, 46, 19, 55, 28],
        [72, 74, 75, 76, 77, 78, 79, 80],
        [64, 65, 72, 74, 54, 55, 56, 63],
    ],
    [
        [65, 2, 38, 11, 47, 20, 56, 29],
        [72, 73, 75, 76, 77, 78, 79, 80],
        [64, 65, 72, 73, 54, 55, 56, 63],
    ],
    [
        [66, 3, 39, 12, 48, 21, 57, 30],
        [72, 73, 74, 76, 77, 78, 79, 80],
        [66, 67, 68, 76, 77, 57, 58, 59],
    ],
    [
        [67, 4, 40, 13, 49, 22, 58, 31],
        [72, 73, 74, 75, 77, 78, 79, 80],
        [66, 67, 68, 75, 77, 57, 58, 59],
    ],
    [
        [32, 68, 5, 41, 14, 50, 23, 59],
        [72, 73, 74, 75, 76, 78, 79, 80],
        [66, 67, 68, 75, 76, 57, 58, 59],
    ],
    [
        [33, 69, 6, 42, 15, 51, 24, 60],
        [72, 73, 74, 75, 76, 77, 79, 80],
        [69, 70, 71, 79, 80, 60, 61, 62],
    ],
    [
        [34, 70, 7, 43, 16, 52, 25, 61],
        [72, 73, 74, 75, 76, 77, 78, 80],
        [69, 70, 71, 78, 80, 60, 61, 62],
    ],
    [
        [35, 71, 8, 44, 17, 53, 26, 62],
        [72, 73, 74, 75, 76, 77, 78, 79],
        [69, 70, 71, 78, 79, 60, 61, 62],
    ],
];

pub const PEER_INDICES: [[usize; 20]; 81] = [
    [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 18, 19, 20, 27, 36, 45, 54, 63, 72,
    ],
    [
        0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 18, 19, 20, 28, 37, 46, 55, 64, 73,
    ],
    [
        0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 18, 19, 20, 29, 38, 47, 56, 65, 74,
    ],
    [
        0, 1, 2, 4, 5, 6, 7, 8, 12, 13, 14, 21, 22, 23, 30, 39, 48, 57, 66, 75,
    ],
    [
        0, 1, 2, 3, 5, 6, 7, 8, 12, 13, 14, 21, 22, 23, 31, 40, 49, 58, 67, 76,
    ],
    [
        0, 1, 2, 3, 4, 6, 7, 8, 12, 13, 14, 21, 22, 23, 32, 41, 50, 59, 68, 77,
    ],
    [
        0, 1, 2, 3, 4, 5, 7, 8, 15, 16, 17, 24, 25, 26, 33, 42, 51, 60, 69, 78,
    ],
    [
        0, 1, 2, 3, 4, 5, 6, 8, 15, 16, 17, 24, 25, 26, 34, 43, 52, 61, 70, 79,
    ],
    [
        0, 1, 2, 3, 4, 5, 6, 7, 15, 16, 17, 24, 25, 26, 35, 44, 53, 62, 71, 80,
    ],
    [
        0, 1, 2, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 27, 36, 45, 54, 63, 72,
    ],
    [
        0, 1, 2, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 28, 37, 46, 55, 64, 73,
    ],
    [
        0, 1, 2, 9, 10, 12, 13, 14, 15, 16, 17, 18, 19, 20, 29, 38, 47, 56, 65, 74,
    ],
    [
        3, 4, 5, 9, 10, 11, 13, 14, 15, 16, 17, 21, 22, 23, 30, 39, 48, 57, 66, 75,
    ],
    [
        3, 4, 5, 9, 10, 11, 12, 14, 15, 16, 17, 21, 22, 23, 31, 40, 49, 58, 67, 76,
    ],
    [
        3, 4, 5, 9, 10, 11, 12, 13, 15, 16, 17, 21, 22, 23, 32, 41, 50, 59, 68, 77,
    ],
    [
        6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 24, 25, 26, 33, 42, 51, 60, 69, 78,
    ],
    [
        6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17, 24, 25, 26, 34, 43, 52, 61, 70, 79,
    ],
    [
        6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 24, 25, 26, 35, 44, 53, 62, 71, 80,
    ],
    [
        0, 1, 2, 9, 10, 11, 19, 20, 21, 22, 23, 24, 25, 26, 27, 36, 45, 54, 63, 72,
    ],
    [
        0, 1, 2, 9, 10, 11, 18, 20, 21, 22, 23, 24, 25, 26, 28, 37, 46, 55, 64, 73,
    ],
    [
        0, 1, 2, 9, 10, 11, 18, 19, 21, 22, 23, 24, 25, 26, 29, 38, 47, 56, 65, 74,
    ],
    [
        3, 4, 5, 12, 13, 14, 18, 19, 20, 22, 23, 24, 25, 26, 30, 39, 48, 57, 66, 75,
    ],
    [
        3, 4, 5, 12, 13, 14, 18, 19, 20, 21, 23, 24, 25, 26, 31, 40, 49, 58, 67, 76,
    ],
    [
        3, 4, 5, 12, 13, 14, 18, 19, 20, 21, 22, 24, 25, 26, 32, 41, 50, 59, 68, 77,
    ],
    [
        6, 7, 8, 15, 16, 17, 18, 19, 20, 21, 22, 23, 25, 26, 33, 42, 51, 60, 69, 78,
    ],
    [
        6, 7, 8, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 26, 34, 43, 52, 61, 70, 79,
    ],
    [
        6, 7, 8, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 35, 44, 53, 62, 71, 80,
    ],
    [
        0, 9, 18, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 45, 46, 47, 54, 63, 72,
    ],
    [
        1, 10, 19, 27, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 45, 46, 47, 55, 64, 73,
    ],
    [
        2, 11, 20, 27, 28, 30, 31, 32, 33, 34, 35, 36, 37, 38, 45, 46, 47, 56, 65, 74,
    ],
    [
        3, 12, 21, 27, 28, 29, 31, 32, 33, 34, 35, 39, 40, 41, 48, 49, 50, 57, 66, 75,
    ],
    [
        4, 13, 22, 27, 28, 29, 30, 32, 33, 34, 35, 39, 40, 41, 48, 49, 50, 58, 67, 76,
    ],
    [
        5, 14, 23, 27, 28, 29, 30, 31, 33, 34, 35, 39, 40, 41, 48, 49, 50, 59, 68, 77,
    ],
    [
        6, 15, 24, 27, 28, 29, 30, 31, 32, 34, 35, 42, 43, 44, 51, 52, 53, 60, 69, 78,
    ],
    [
        7, 16, 25, 27, 28, 29, 30, 31, 32, 33, 35, 42, 43, 44, 51, 52, 53, 61, 70, 79,
    ],
    [
        8, 17, 26, 27, 28, 29, 30, 31, 32, 33, 34, 42, 43, 44, 51, 52, 53, 62, 71, 80,
    ],
    [
        0, 9, 18, 27, 28, 29, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 54, 63, 72,
    ],
    [
        1, 10, 19, 27, 28, 29, 36, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 55, 64, 73,
    ],
    [
        2, 11, 20, 27, 28, 29, 36, 37, 39, 40, 41, 42, 43, 44, 45, 46, 47, 56, 65, 74,
    ],
    [
        3, 12, 21, 30, 31, 32, 36, 37, 38, 40, 41, 42, 43, 44, 48, 49, 50, 57, 66, 75,
    ],
    [
        4, 13, 22, 30, 31, 32, 36, 37, 38, 39, 41, 42, 43, 44, 48, 49, 50, 58, 67, 76,
    ],
    [
        5, 14, 23, 30, 31, 32, 36, 37, 38, 39, 40, 42, 43, 44, 48, 49, 50, 59, 68, 77,
    ],
    [
        6, 15, 24, 33, 34, 35, 36, 37, 38, 39, 40, 41, 43, 44, 51, 52, 53, 60, 69, 78,
    ],
    [
        7, 16, 25, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 44, 51, 52, 53, 61, 70, 79,
    ],
    [
        8, 17, 26, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 51, 52, 53, 62, 71, 80,
    ],
    [
        0, 9, 18, 27, 28, 29, 36, 37, 38, 46, 47, 48, 49, 50, 51, 52, 53, 54, 63, 72,
    ],
    [
        1, 10, 19, 27, 28, 29, 36, 37, 38, 45, 47, 48, 49, 50, 51, 52, 53, 55, 64, 73,
    ],
    [
        2, 11, 20, 27, 28, 29, 36, 37, 38, 45, 46, 48, 49, 50, 51, 52, 53, 56, 65, 74,
    ],
    [
        3, 12, 21, 30, 31, 32, 39, 40, 41, 45, 46, 47, 49, 50, 51, 52, 53, 57, 66, 75,
    ],
    [
        4, 13, 22, 30, 31, 32, 39, 40, 41, 45, 46, 47, 48, 50, 51, 52, 53, 58, 67, 76,
    ],
    [
        5, 14, 23, 30, 31, 32, 39, 40, 41, 45, 46, 47, 48, 49, 51, 52, 53, 59, 68, 77,
    ],
    [
        6, 15, 24, 33, 34, 35, 42, 43, 44, 45, 46, 47, 48, 49, 50, 52, 53, 60, 69, 78,
    ],
    [
        7, 16, 25, 33, 34, 35, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 53, 61, 70, 79,
    ],
    [
        8, 17, 26, 33, 34, 35, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 62, 71, 80,
    ],
    [
        0, 9, 18, 27, 36, 45, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 72, 73, 74,
    ],
    [
        1, 10, 19, 28, 37, 46, 54, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 72, 73, 74,
    ],
    [
        2, 11, 20, 29, 38, 47, 54, 55, 57, 58, 59, 60, 61, 62, 63, 64, 65, 72, 73, 74,
    ],
    [
        3, 12, 21, 30, 39, 48, 54, 55, 56, 58, 59, 60, 61, 62, 66, 67, 68, 75, 76, 77,
    ],
    [
        4, 13, 22, 31, 40, 49, 54, 55, 56, 57, 59, 60, 61, 62, 66, 67, 68, 75, 76, 77,
    ],
    [
        5, 14, 23, 32, 41, 50, 54, 55, 56, 57, 58, 60, 61, 62, 66, 67, 68, 75, 76, 77,
    ],
    [
        6, 15, 24, 33, 42, 51, 54, 55, 56, 57, 58, 59, 61, 62, 69, 70, 71, 78, 79, 80,
    ],
    [
        7, 16, 25, 34, 43, 52, 54, 55, 56, 57, 58, 59, 60, 62, 69, 70, 71, 78, 79, 80,
    ],
    [
        8, 17, 26, 35, 44, 53, 54, 55, 56, 57, 58, 59, 60, 61, 69, 70, 71, 78, 79, 80,
    ],
    [
        0, 9, 18, 27, 36, 45, 54, 55, 56, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74,
    ],
    [
        1, 10, 19, 28, 37, 46, 54, 55, 56, 63, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74,
    ],
    [
        2, 11, 20, 29, 38, 47, 54, 55, 56, 63, 64, 66, 67, 68, 69, 70, 71, 72, 73, 74,
    ],
    [
        3, 12, 21, 30, 39, 48, 57, 58, 59, 63, 64, 65, 67, 68, 69, 70, 71, 75, 76, 77,
    ],
    [
        4, 13, 22, 31, 40, 49, 57, 58, 59, 63, 64, 65, 66, 68, 69, 70, 71, 75, 76, 77,
    ],
    [
        5, 14, 23, 32, 41, 50, 57, 58, 59, 63, 64, 65, 66, 67, 69, 70, 71, 75, 76, 77,
    ],
    [
        6, 15, 24, 33, 42, 51, 60, 61, 62, 63, 64, 65, 66, 67, 68, 70, 71, 78, 79, 80,
    ],
    [
        7, 16, 25, 34, 43, 52, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 71, 78, 79, 80,
    ],
    [
        8, 17, 26, 35, 44, 53, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 78, 79, 80,
    ],
    [
        0, 9, 18, 27, 36, 45, 54, 55, 56, 63, 64, 65, 73, 74, 75, 76, 77, 78, 79, 80,
    ],
    [
        1, 10, 19, 28, 37, 46, 54, 55, 56, 63, 64, 65, 72, 74, 75, 76, 77, 78, 79, 80,
    ],
    [
        2, 11, 20, 29, 38, 47, 54, 55, 56, 63, 64, 65, 72, 73, 75, 76, 77, 78, 79, 80,
    ],
    [
        3, 12, 21, 30, 39, 48, 57, 58, 59, 66, 67, 68, 72, 73, 74, 76, 77, 78, 79, 80,
    ],
    [
        4, 13, 22, 31, 40, 49, 57, 58, 59, 66, 67, 68, 72, 73, 74, 75, 77, 78, 79, 80,
    ],
    [
        5, 14, 23, 32, 41, 50, 57, 58, 59, 66, 67, 68, 72, 73, 74, 75, 76, 78, 79, 80,
    ],
    [
        6, 15, 24, 33, 42, 51, 60, 61, 62, 69, 70, 71, 72, 73, 74, 75, 76, 77, 79, 80,
    ],
    [
        7, 16, 25, 34, 43, 52, 60, 61, 62, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 80,
    ],
    [
        8, 17, 26, 35, 44, 53, 60, 61, 62, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    ],
];
