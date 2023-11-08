static test: int = 1 + a / (2 * cat);
test = testing;

static function main(): void {
    static loaded: int = test;
    {
        // This is a comment
        static scoped: int = 0i;
    }
    tick();
    command("say loading!");
}

static function tick(): void {
    command("say tick!");
}