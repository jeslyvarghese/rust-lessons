fn main() {
  static MONSTER_FACTOR: float = 23.45; //since this is a static var, type specification is re	uired
  let monster_size = MONSTER_FACTOR * 4.5;
  //here its not required
  let another_monster_size: int = 30;
  //its just optional, nothing stops from specifying the type.
}
