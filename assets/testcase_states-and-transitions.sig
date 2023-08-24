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
  }
}
transition unshuffled to shuffled {
  return {
  };
}
transition lose to quit {
  return {
  };
}
transition win to quit {
  return {
  };
}
