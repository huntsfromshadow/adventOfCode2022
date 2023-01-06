direcs = {}
pwd = []

def handle_file(size, name):
    print("handle file -- ", name, " -- ", size)
    lpwd = pwd.copy()
    total_direc(lpwd, size)
    print("================")

def total_direc(lpwd, size):
    print("total direc - ", lpwd, " -- ", size)

    ll = "/" + "/".join(lpwd)
    if(direcs.get(ll) == None):
        direcs[ll] = 0

    direcs[ll] = direcs[ll] + size

    if(len(lpwd) == 0):
        return
    else:
        lpwd.pop()
        total_direc(lpwd, size)

f = open("input.txt", "r")
lines = f.read().splitlines()

for l in lines:
    if( l.startswith("$ cd")):
        d = l.split(" ")
        if(d[2] == "/"):
            pwd = []
        elif (d[2] == ".."):
            pwd.pop()
        else:
            pwd.append(d[2])

    elif( l.startswith("$ ls")):
        pass
    else:
        d = l.split(" ")
        if(d[0] != "dir"):
            handle_file(int(d[0]), d[1])

total_space = 70000000
used_space = direcs["/"]
cur_unused = total_space - used_space

unusued_space_needed = 30000000
need_to_find = unusued_space_needed - cur_unused

print(used_space)
print(cur_unused)
print(need_to_find)

lst = []
for k,v in direcs.items():
    if( v >= need_to_find ):
        lst.append((v))

print(lst)
print(min(lst))