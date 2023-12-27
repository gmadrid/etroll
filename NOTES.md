# Why am I doing this?

Right now, this utility is to help me with my options trading. I want it to:
- Find all of my "weekly" trades (diagonal puts or covered calls with a short-term weekly component.)
- Evaluate my current maximum loss, including:
    - "gap" in dollars between the target prices of the two legs
    - that gap times the number of shares controlled (contracts * 100)
- Tell me current percentage gain for the short-term legs
    - If Mon or Tues, then a 90% threshold should be enough to suggest a roll.
        - compute a roll into a current week higher target, or
        - compute a roll into the next week at a higher target
        - (It would be nice to learn how to evaluate the risk of exercise and display expected values)
    - If Wed or Thurs, then a 95% threshold will suggest a roll
        - compute an early roll into next week
    - On Friday, suggest rolls into next week.
- For all suggested rolls,
    - give the profit from the closed contract
    - give the price of the opened contract
    - total cost of the trade
    - profit percentage in some metric that makes sense (See the Fool to determine this.)

Longer term:
- For a stock symbol, troll the options schedule for profitable contracts
    - using percentage exercise
    - and percentage gain
- For portfolio, find stocks with potential covered calls and find profitable contracts.

