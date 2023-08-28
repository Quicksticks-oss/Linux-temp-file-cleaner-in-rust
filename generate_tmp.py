
for x in range(128):
    path = f'/tmp/{x}.tmp'
    with open(path, 'w+') as f:
        f.write(f'{x}')
    print(f'Generated {path}')