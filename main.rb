#typed: true
class Hello
  extend T::Sig

  sig { returns String }
  def world
    "World"
  end
  def hi
    print "Hello, " + world
  end
end
