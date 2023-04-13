program elf_calories
  use iso_fortran_env
  implicit none

  integer :: i, stat, line_value, total_calories, most_calories, elf_number, fattest_elf
  character(len=100) :: line

  elf_number = 0
  most_calories = 0
  total_calories = 0

  open(0, file="../input.txt")
  do while(stat == 0)
    read(0, "( a )", iostat=stat, blank="ZERO") line
    if (line == "") then
      if (total_calories > most_calories) then
        most_calories = total_calories
        fattest_elf = elf_number
      end if
      total_calories = 0
      elf_number = elf_number + 1
    else
      read(line, *, iostat=stat) line_value
      total_calories = total_calories + line_value
    end if
  enddo
  close(0)

  print *, "Elf number ", fattest_elf, " carries ", most_calories, " calories."
end program elf_calories
