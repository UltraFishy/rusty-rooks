![Rusty-rooks](./assets/♖.png)

# RUSTY-ROOKS

## Overview

Rusty Rooks is a community-driven chess application where teams collaborate and vote on their next moves. Built in Rust, this application ensures a fair and interactive experience for all players involved. Connect via SSH or netcat (nc) to participate in the game and contribute to your team's strategy.

## Features

- **Community Voting**: Teams vote on the next move, ensuring a democratic decision-making process.
- **Real-time Updates**: The board is updated in real-time, reflecting the collective decision of the team.
- **Accessible Interface**: Connect easily via SSH or netcat to join the game from anywhere.
- **Secure and Robust**: Built with Rust for performance and safety.

## Getting Started

### Prerequisites

- Rust installed on your machine. You can install it from [here](https://www.rust-lang.org/tools/install).
- SSH or netcat (nc) installed.

### Installation

Clone the repository:

```bash
git clone https://github.com/UltraFishy/rusty-rooks.git
cd rusty-rooks
```
### Connecting 
```bash
nc rusty-rooks.game 2345
```
