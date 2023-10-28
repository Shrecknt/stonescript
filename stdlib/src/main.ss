static RETURN_VALUE: int;

# TODO: initialize scoreboard, create pointer armor stand
# The pointer is an armor stand named "__stdlib__pointer"

static function move_pointer_to(address: int) {
    # TODO
    static address: int = address;
    $execute store result entity @e[tag=pointer] Pos[0] double 1 run scoreboard players get __stdlib__pointer_address ss_global;
}

static function read_pointer_value() {
    static pointer_value: int;
    # read data at the pointer's location into `pointer_value`
}

# Allocates `size` bytes in memory and returns the address of the allocated memory
static function alloc(size: int): int {
    # TODO
}

# De-allocates the memory at the address `address`
static function dealloc(address: int, size: int) {
    # TODO
}

# Reads the value at the address `address` into the `return` register
static function read(address: int): int {
    static pointer_value: int;
    move_pointer_to(address);
    read_pointer_value();
    RETURN_VALUE = pointer_value;
}

# Writes the value from `data` to the memory at `address`
static function write(address: int, data: int) {
    # TODO
    move_pointer_to(address);
    $execute at @e[tag=pointer] run setblock ~ ~ ~ stone;
}
