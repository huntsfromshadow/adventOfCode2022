class Drive:
    pwd = ["/"]
    data = {}

    def add_directory(self, name):
        self.data[name] = (True, name, 0, {})

    def add_file(self, name, size):
        self.data[name] = (False, name, size, None)

    def change_direc(self, cd):
        if(cd == "/"):
            self.pwd = ["/"]
        elif(cd == ".."):
            self.pwd.pop()
        else:
            self.pwd.append(cd)

    def get_dir(self):
        local = None
        for p in pwd:
            if( p == "/"):
                local = self.data
            else:
                local = local[3][p]
                print("Setting local to ", local)

    







#######################################

def process_command(line, drive):
    if(line == "$ ls"):
        drive.load_dir_pointer()
    else:
        c = line.split(" ")[2]
        drive.change_direc(c)        

#######################

f = open("short_input.txt", "r")
lines = f.read().splitlines()
d = Drive()

for l in lines:

    if( l.startswith("$") ):
        process_command(l, d)
    else:
        dat = l.split(" ")
        if(dat[0] == "dir"):
            d.add_directory(dat[1])
        else:
            d.add_file(dat[1], dat[0])


        #d.print_drive()
        #print("===========")

