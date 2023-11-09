static test: int = 1 + a / (2 * cat);
test = testing;

static function main(): int {
    static loaded: int = test;
    {
        // This is a comment
        static scoped: float = 0.0;
    }
    tick();
    command("say loading!");

    return 1;
}

static function tick(): void {
    command("say tick!");
}