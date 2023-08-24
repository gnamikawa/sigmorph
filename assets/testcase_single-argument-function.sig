trait Enumerable<T> {
  T next;
}
trait Indexable<T> {
  T at(int index);
}
trait Shuffleable {
  Indexable shuffle(Indexable elements);
}
trait Sortable {
  Indexable shuffle(Indexable elements);
}

enum Suit {
  Clubs,
  Hearts,
  Spades,
  Diamonds
}
struct Card {
  Suit suit;
  int rank;
}
struct DeckOfCards extends Array<Card>; 

impl Shuffleable for DeckOfCards {
  Indexable shuffle(this DeckOfCards cards){
    return cards
  }
}

state main {};
state shuffled {
  DeckOfCards deckOfCards;
};
state play {
  DeckOfCards deckOfCards;
};
state win {
  Card[] winningHand;
}
state lose {}
state quit {
  String reason;
}

transition main to shuffled {
  return {
    deckOfCards: DeckOfCards{}.shuffle()
  }
}
transition unshuffled to shuffled {
  return {
    deckOfCards: input.shuffle()
  };
}
transition lose to quit {
  return {
    reason: "I hate this game"
  };
}
transition win to quit {
  return {
    reason: "I love this game!"
  };
}
