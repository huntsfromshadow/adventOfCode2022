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

total = 0
for k,v in direcs.items():
    if(v <= 100000):
        print(k)
        total = total + v

print(total)


# direcs = []
# files = {}
# pwd = None
# master_total = 0

# def process_command(line):
#     global pwd

#     if(line != "$ ls"):
#         c = line.split(" ")
#         if(c[2] == "/"):
#             print("Setting pwd")
#             pwd = ["/"]
#             print(pwd)
#         elif(c[2] == ".."):
#             pwd.pop()
#         else:
#             pwd.append(c[2])

# def add_file(name, size):
#     p = make_pwd_path()

#     global direcs
#     global files

#     fn = p + "/" + name
#     fn = fn.replace("//", "/")

#     global master_total
#     master_total = master_total + int(size)

#     ft = (p, fn, int(size))
#     files[fn] = ft

#     if( (p in direcs) == False):
#         direcs.append(p)

# def make_pwd_path():
#     global pwd
#     lpwd = pwd.copy()
#     print (lpwd)
#     if(len(lpwd) == 1):
#         return "/"
#     else:
#         lpwd.pop(0)
#         return '/' + '/'.join(lpwd)

# #######################

# f = open("input.txt", "r")
# lines = f.read().splitlines()

# for l in lines:

#     if( l.startswith("$") ):
#         process_command(l)
#     else:
#         dat = l.split(" ")
#         if(dat[0] != "dir"):
#             add_file(dat[1], dat[0])

# # direcs.sort()

# print("=================================")
# sizes = {}
# for d in direcs:
#     #print("Looking at ",d)
#     if(sizes.get(d) == None):
#         sizes[d] = 0

#     for f in files.values():
#         p = f[0]

#         print("Comparing ~",d,"~ against ~", p, " Result -> ", p.startswith(d))

#         if(p.startswith(d)):
#             #print(f)
#             sizes[d] = sizes[d] + f[2]

# print("==============")

# print(sizes)
# print(master_total)

# total = 0
# for k,v in sizes.items():
#     if v <= 100000:
#         print("-------- ", k, v)
#         total = total + v
#     else:
#         print("Skipping ", k, " ", v)


# #print("Res ", total)