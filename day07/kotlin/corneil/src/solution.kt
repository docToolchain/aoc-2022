fun main() {
  val test = readFile("day07_test")
  val input = readFile("day07")

  data class FileInfo(val name: String, val size: Int)
  class Directory(val name: String) {
    private val files = mutableMapOf<String, FileInfo>()
    private val directories = mutableMapOf<String, Directory>()
    fun listDirectories() = directories.values.toList()

    fun hasDir(dirname: String): Boolean = 
            directories.containsKey(dirname)
    fun getDir(dirname: String) = directories[dirname]
    fun addDir(dirname: String): Directory = 
            directories.getOrPut(dirname) { Directory(dirname) }
    fun addFile(name: String, size: Int): FileInfo {
      val file = FileInfo(name, size)
      files[name] = file
      return file
    }

    fun listFiles(): List<FileInfo> = files.values.toList()
    fun fileSizes(): Int = files.values.sumOf { it.size }
    fun totalSize(): Int = 
            fileSizes() + directories.values.sumOf { it.totalSize() }
    override fun toString(): String {
      return "Directory(name='$name',directories=${
        directories.values.map { it.name }.toList()
      },files=${files.values.map { "${it.name}:${it.size}" }})"
    }

    override fun equals(other: Any?): Boolean {
      if (this === other) return true
      if (javaClass != other?.javaClass) return false

      other as Directory

      if (name != other.name) return false

      return true
    }

    override fun hashCode(): Int {
      return name.hashCode()
    }

    private fun printTree(dir: Directory, depth: Int) {
      println("- ${dir.name} (dir)")
      val prefix = 0 until depth
      dir.listDirectories().forEach {
        prefix.forEach { _ -> print(' ') }
        printTree(it, depth + 2)
      }
      dir.listFiles().forEach {
        prefix.forEach { _ -> print(' ') }
        println("- ${it.name} (file, size = ${it.size})")
      }
    }

    fun printTree() = printTree(this, 0)
  }

  fun listAllDirectories(directories: Directory): List<Directory> {
    val result = mutableListOf<Directory>()
    directories.listDirectories().forEach {
      result.add(it)
      result.addAll(listAllDirectories(it))
    }
    return result
  }

  fun parseCommandsAndOutput(input: List<String>): Directory {
    val root = Directory("/")
    val dirs = Stack<Directory>()
    dirs.push(root)
    var current = root
    input.forEach { line ->
      val data = line.split(" ")
      when {
        data[0] == "$" && data[1] == "cd" && data[2] == "/" -> { dirs.clear(); dirs.push(root) }
        data[0] == "$" && data[1] == "cd" && data[2] == ".." -> { dirs.pop(); current = dirs.peek()}
        data[0] == "$" && data[1] == "cd" -> { dirs.push(current.addDir(data[2])); current = dirs.peek() }
        data[0] == "$" && data[1] == "ls" -> println("listing ${dirs.items().joinToString("/") { it.name }}")
        data[0] == "dir" -> current.addDir(data[1])
        else -> {
          val file = current.addFile(data[1], data[0].toInt())
          println("file=dir:${current.name} + ${file.name}:${file.size}")
        }
      }
    }
    return root
  }


  fun calcDirectoriesBelow(input: List<String>, maxValue: Int): Int {
    val root = parseCommandsAndOutput(input)
    root.printTree()
    val directories = listAllDirectories(root)
    return directories.map { it.totalSize() }.filter { it <= maxValue }.sum()
  }

  fun calcDirectoriesToFree(input: List<String>, diskSize: Int, freeSpace: Int): Int {
    val root = parseCommandsAndOutput(input)
    val directories = listAllDirectories(root)
    val unused = diskSize - root.totalSize()
    val requiredDelete = freeSpace - unused
    return directories.map { it.totalSize() }.filter { it > requiredDelete }.min()
  }


  fun part1() {
    val testResult = calcDirectoriesBelow(test, 100000)
    println("Part 1 Answer = $testResult")
    check(testResult == 95437)
    separator()
    val result = calcDirectoriesBelow(input, 100000)
    check(result == 1792222)
    println("Part 1 Answer = $result")
    separator()
  }

  fun part2() {
    val testResult = calcDirectoriesToFree(test, 70000000, 30000000)
    println("Part 2 Answer = $testResult")
    check(testResult == 24933642)
    separator()
    val result = calcDirectoriesToFree(input, 70000000, 30000000)
    println("Part 2 Answer = $result")
    check(result == 1112963)
    separator()
  }
  println("Day - 07")
  part1()
  part2()
}
