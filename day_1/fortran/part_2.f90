program elf_calories
  use iso_fortran_env
  implicit none

  integer :: i, check_zero, stat, line_value, total_calories, elf_number
  integer, dimension(3) :: most_calories = 0, fattest_elves = 0
  character(len=100) :: line

  elf_number = 0
  most_calories = 0
  total_calories = 0

  open(0, file="../input.txt")
  do while(stat == 0)
    read(0, "( a )", iostat=stat, blank="ZERO") line
    if (line == "") then
      if (total_calories > 65000) print *, total_calories
      if (total_calories > most_calories(3)) then 
        if (total_calories > most_calories(2)) then
          if (total_calories > most_calories(1)) then
            most_calories(2) = most_calories(1)
            most_calories(1) = total_calories
            fattest_elves(1) = elf_number
          else
            most_calories(3) = most_calories(2)
            most_calories(2) = total_calories
            fattest_elves(2) = elf_number
          endif
        else
          most_calories(3) = total_calories
          fattest_elves(3) = elf_number
        endif
      endif
      total_calories = 0
      elf_number = elf_number + 1
    else
      read(line, *, iostat=stat) line_value
      total_calories = total_calories + line_value
    endif
  enddo
  close(0)

  print *, "The fattest elves are: ", fattest_elves
  print *, "They carry the following amount of calories: ", most_calories
  print *, "Which in total equals to " , sum(most_calories), " calories."
end program elf_calories
