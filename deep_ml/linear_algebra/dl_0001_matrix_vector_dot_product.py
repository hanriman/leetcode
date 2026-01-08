def matrix_dot_vector(a: list[list[int|float]], b: list[int|float]) -> list[int|float] | int:
	# Return a list where each element is the dot product of a row of 'a' with 'b'.
	# If the number of columns in 'a' does not match the length of 'b', return -1.
    if len(a[0]) != len(b):
        return -1
    
    res = []
    for row in a:
        dot_product = 0
        for i in range(len(b)):
            dot_product += row[i] * b[i]
        res.append(dot_product)    
    
    return res


def test_basic():
    assert matrix_dot_vector([[1, 2], [2, 4]], [1, 2]) == [5, 10]

def test_dim_mismatch():
    assert matrix_dot_vector([[1, 2, 3]], [1, 2]) == -1


if __name__ == "__main__":
    test_basic()
    test_dim_mismatch()
    print("All tests passed")

