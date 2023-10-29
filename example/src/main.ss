static test: int = testing;

static function main(): void {
    static loaded: int = test;
    $say "loading!";
}

static function tick(): void {
    $say "loading!";
}