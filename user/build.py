import os

base_addr = 0x80400000
step = 0x20000
linker = 'src/linker.ld'

app_id = 0
apps = os.listdir('src/bin')
apps.sort() # order by number

for app in apps:
    app = app[:app.find('.')]
    lines = []
    lines_origin = []
    new_base_addr = base_addr + step * app_id
    with open(linker, 'r') as f:
        for line in f.readlines():
            lines_origin.append(line)
            line = line.replace(hex(base_addr), hex(new_base_addr))
            lines.append(line)
    with open(linker, 'w+') as f:
        f.writelines(lines)
    print('[build.py] application %s start with base_addr: %s' %(app, hex(new_base_addr)))
    os.system('cargo build --bin %s --release' % app)
    with open(linker, 'w+') as f:
        f.writelines(lines_origin)
    app_id += 1

