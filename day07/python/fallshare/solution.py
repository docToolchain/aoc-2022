
class Folder:
    folders = {}
    files = []
    def __init__(self, name, parent):
        self.name = name
        self.parent = parent
 
    def get_folder(self, folder):
         return self.folders[folder]

    def add_file(self, file):
        self.files.append(file)

    def add_folder(self,folder):
        self.folders[folder.name] = Folder(folder, self)

    def get_size(self):
        size = 0
        for file in self.files:
            size += file.get_size()
        print(self.folders.keys())
        for folder in self.folders.values():
            size += folder.get_size()
        return size

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
        print(f"{instruction} -- {result}")

        if instruction[0] == "cd":
            print(current_node.folders.keys())
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
                    current_node.add_folder(Folder(object[1], current_node))
                else:
                    current_node.add_file(File(object[1], int(object[0])))



    
        


if __name__ == '__main__':
    main()


