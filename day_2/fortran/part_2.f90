module easydict
  implicit none

  type :: dict
    character(2), dimension(:), allocatable :: key
    integer, dimension(:), allocatable :: val
    contains
      procedure :: get => get_value
  end type

contains 
  pure integer function get_value(self, input) result(val)
    class(dict), intent(in) :: self
    character(*), intent(in) :: input
    
    val = self%val(findloc(self%key, input, dim=1))
  end function get_value
end module easydict

program rock_paper_scissors
  use easydict
  use iso_fortran_env
  implicit none

  integer :: stat, score = 0
  character(len=1) :: oppo, result_
  character(len=2) :: player_shape
  type(dict) :: outcomes, shapes

  ! A = Rock, B = Paper, C = Scissors
  ! X = Loss(0), Y = Draw(3), Z = Win(6)

  outcomes = dict(["X", "Y", "Z"], [0, 3, 6])
  shapes = dict(                                            &
    ["AX", "AY", "AZ", "BX", "BY", "BZ", "CX", "CY", "CZ"], &
    [   3,    1,    2,    1,    2,    3,    2,    3,    1])

  open(0, file="../input.txt")
  do 
    read(0, *, iostat=stat) oppo, result_
    if (stat /= 0) exit
    player_shape = oppo//result_
    score = score + outcomes%get(result_) + shapes%get(player_shape)
  enddo
  close(0)

  print *, "Final score = ", score
end program rock_paper_scissors 
