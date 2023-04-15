module easydict
  implicit none

  type :: dict
    character(2), dimension(:), allocatable :: key
    integer, dimension(:), allocatable :: val
    contains
      procedure :: get => get_value
  end type

contains 
  integer function get_value(self, input) result(val)
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
  character(len=1) :: oppo, player
  character(len=1), dimension(3) :: shapes = ["X", "Y", "Z"]
  character(len=2) :: result_
  type(dict) :: outcomes

  ! A = Rock, B = Paper, C = Scissors
  ! X = Rock, Y = Paper, Z = Scissors
  outcomes = dict(2,                                        &
    ["AX", "AY", "AZ", "BX", "BY", "BZ", "CX", "CY", "CZ"], &
    [   3,    6,    0,    0,    3,    6,    6,    0,    3])

  open(0, file="../input.txt")
  do
    read(0, *, iostat=stat) oppo, player
    if (stat /= 0) exit
    result_ = oppo//player
    score = score + findloc(shapes, player, dim=1) + outcomes%get(result_)
  enddo
  close(0)

  print *, "Final score = ", score
end program rock_paper_scissors 
