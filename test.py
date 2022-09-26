import os

for i in range(500):
    print(i * 5000)
    os.system(f"/usr/bin/time -a -o data.csv -f '%e,%C' ./target/release/prime {i * 5000}")
