
class Folder:
    def __init__(self, name, parent):
        self.name = name
        self.parent = parent
        self.folders = {}
        self.files = []
        self.size = 0
 
    def printContent(self):
        print(f"{self.name}")
        print("children:")
        for folder in self.folders.values():
            folder.printContent()
        for file in self.files:
            print(f"{file.name}: {file.get_size()}")

    def get_folder(self, folder):
         return self.folders[folder]

    def get_folders(self):
        return self.folders  

    def add_file(self, file):
        self.files.append(file)

    def add_folder(self,folder):
        self.folders[folder.name] = folder

    def get_size(self):
        size = 0
        for file in self.files:
            size += file.get_size()
        for folder in self.folders.values():
            size += folder.get_size()
        return size

    def get_sizes_of_directories(self):
        sizes = []
        for folder in self.folders.values():
            sizes += folder.get_sizes_of_directories()

        sizes.append(self.get_size())
        return sizes



class File:
    def __init__(self, name, size):
        self.name = name
        self.size = size
        
    def get_size(self):
        return self.size
 

def main():
    root = Folder('root', None)

    file = open('input.txt', 'r')
    commands = file.read().split("$")
    commands.pop(0)

    current_node = root

    for command in commands:
        command = command.splitlines()
        instruction = command.pop(0).split()
        result = command

        if instruction[0] == "cd":

            if instruction[1] == "/":
                current_node = root
            elif instruction[1] == "..":
                current_node = current_node.parent
            else:
                current_node = current_node.get_folder(instruction[1])

        if instruction[0] == "ls":
            for object in result:
                object = object.split()
                if object[0] == 'dir':
                    myFolder = Folder(object[1], current_node)
                    current_node.add_folder(myFolder)

                else:
                    current_node.add_file(File(object[1], int(object[0])))

    # root.printContent()
    sizes = root.get_sizes_of_directories()

    sum = 0
    for size in sizes:
        if size <= 100000:
            sum += size
    print(f"Solution Star 1 is {sum}")


    currently_free = 70000000 - root.get_size()
    print(f"Currently {currently_free} are free.")
    required_space = 30000000 - currently_free
    print(f"At least {required_space} must be removed.")

    smallest_folder_to_delete = None
    smallest_extra_space = 70000000
    for size in sizes:
        extra_unused = size - required_space
        if extra_unused >= 0:
            if extra_unused < smallest_extra_space:
                smallest_extra_space = extra_unused
                smallest_folder_to_delete = size
            
    print(f"Star 2: The smalles folder to delete is: {smallest_folder_to_delete}")
       

if __name__ == '__main__':
    main()
