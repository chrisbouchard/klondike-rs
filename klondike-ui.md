# Klondike-rs UI Design

The UI will use the block drawing and card suite (♠ ♥ ♦ ♣) Unicode symbols.


## Card Design

    1.          2.          3.
    ╭────────╮  ╭────────╮  ╭────────╮
    │ A    ♠ │  │ 10   ♥ │  │ ░░░░░░ │
    │ ♠    A │  │ ♥   10 │  │ ░░░░░░ │
    ╰────────╯  ╰────────╯  ╰────────╯

1. Each card is a 10 wide by 4 tall box with rounded corners. Wider than tall
   because that's easier to fit in the terminal.
2. The tens need two columns to display their rank.
3. Face-down cards just show a cardback. Maybe make this configurable?


## Stock Area Design

    1. ╭╭────────╮   ╭╭───╭───╭────────╮
       ││ ░░░░░░ │   ││ A │ 2 │ 3    ♦ │
       ││ ░░░░░░ │   ││ ♠ │ ♥ │ ♦    3 │
       ╰╰────────╯   ╰╰───╰───╰────────╯

    2. ╭╭────────╮   ╭╭───╭────╭────────╮
       ││ ░░░░░░ │   ││ A │ 2  │ 3    ♦ │
       ││ ░░░░░░ │   ││ ♠ │ ♥  │ ♦    3 │
       ╰╰────────╯   ╰╰───╰────╰────────╯
                               ╘════════╛

    3. ╭╭────────╮   ╭╭────────╮
       ││ ░░░░░░ │   ││ A    ♠ │
       ││ ░░░░░░ │   ││ ♠    A │
       ╰╰────────╯   ╰╰────────╯

    4. ╭╭────────╮   ╭─╭────────╮
       ││ ░░░░░░ │   │ │ A    ♠ │
       ││ ░░░░░░ │   │ │ ♠    A │
       ╰╰────────╯   ╰─╰────────╯
                       ╘════════╛

    5. ╭─╭────────╮  ╭╭────────╮
       │ │ ░░░░░░ │  ││ A    ♠ │
       │ │ ░░░░░░ │  ││ ♠    A │
       ╰─╰────────╯  ╰╰────────╯
         ╘════════╛

1. The face-down stock and three revealed cards. The cards drawn from the stock
   form a horizontal card stack. If there are more than three cards revealed,
   the remainder form a pile under the stock. Their faces are not visible.
2. Only the top-most (right-most) card can be selected. In addition to showing
   the selector, selecting shifts the card right one character.
3. Once the top three cards are removed, the remaining cards form a face-up
   pile with only the top card's face visible.
4. Selecting the top of the pile still shifts the card right one character.
5. Selecting the stock shifts the top card.


## Tableaux Stack Design

    1.           2.           3.
    ╭────────╮   ╭────────╮   ╭────────╮
    │ ░░░░░░ │   │ ░░░░░░ │   │ ░░░░░░ │
    ╭────────╮   ╭────────╮   ╭────────╮
    │ K    ♠ │   │ K    ♠ │   │ K    ♠ │
    ╭────────╮  ╓│╭────────╮  │╭────────╮
    │ Q    ♦ │  ║╰│ Q    ♦ │  ╰│ Q    ♦ │
    ╭────────╮  ║ ╭────────╮   ╭────────╮
    │ J    ♣ │  ║ │ J    ♣ │   │ J    ♣ │
    │ ♣    J │  ║ │ ♣    J │   │ ♣    J │
    ╰────────╯  ╙ ╰────────╯   ╰────────╯
                               ╘════════╛

1. Vertical tableaux stacks are drawn with two rows of each card visible, and
   all four rows of the top card visible (lower cards are on top of higher
   cards).
2. Selecting a sub-stack displays a vertical selector and shifts the sub-stack
   right one character.
3. While moving a sub-stack, the sub-stack remains shifted right one character,
   but the selector becomes a horizontal selector.


## Open Design Questions

1. How should we display empty areas? Most of the time an empty area isn't a
   selectable object -- e.g., empty tableaux stacks, empty foundation piles --
   but some are -- e.g., the empty stock, which should refill it.
   * We could just show the selector under the empty spot. This is consistent
     but not very discoverable.
2. It's difficult to distinguish between Spades (♠) and Clubs (♣), at least in
   my font (Iosevka). Is this an issue and can we make it better?

