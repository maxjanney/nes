pal_path = "..\\..\\Downloads\\pal.pal"

with open(pal_path, "rb") as p:
    data = p.read()

data = [data[i:i+3] for i in range(0, len(data), 3)]

with open("pal.txt", "x") as p:
    for c in data:
        x, y, z = c
        p.write(f"({x}, {y}, {z}),\n")
        
