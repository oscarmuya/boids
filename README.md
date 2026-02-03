# Boids Simulation in Rust

This project is a classic Boids simulation implemented in Rust using the Bevy game engine. It demonstrates the emergent behavior of a flock of birds, where simple local rules create complex, life-like patterns.

![Boids Screenshot](httpsio.io)

## What are Boids?

Boids is an artificial life program, developed by Craig Reynolds in 1986, which simulates the flocking behavior of birds. The name "boid" corresponds to "bird-oid object".

The simulation is based on three simple rules that each boid follows:

1.  **Separation:** Steer to avoid crowding local flockmates.
2.  **Alignment:** Steer towards the average heading of local flockmates.
3.  **Cohesion:** Steer to move toward the average position (center of mass) of local flockmates.

By following these rules, the flock exhibits emergent behavior, creating complex and realistic-looking flocking patterns.

## Dependencies

-   [Rust](https://www.rust-lang.org/)
-   [Bevy Game Engine](https://bevyengine.org/)

## Getting Started

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/oscarmuya/boids.git
    cd boids
    ```
2.  **Run the simulation:**
    ```bash
    cargo run --release
    ```
    The `--release` flag is recommended for better performance.

## Implementation

The project is structured as a Bevy application, with the following key components:

-   `main.rs`: The entry point of the application. It sets up the Bevy app, the game window, and registers the necessary systems.
-   `components.rs`: Defines the `Boid` and `Velocity` components that are attached to each boid entity.
-   `systems.rs`: Contains the core logic of the simulation.
    -   `setup_boids`: Initializes the simulation by spawning a number of boids with random positions and velocities.
    -   `move_boids`: This system is run on every frame and updates the position and velocity of each boid based on the three Boids rules. It uses a spatial hash to efficiently find neighboring boids.
-   `helpers.rs`: Contains helper functions, such as for calculating if a point is within an arc, used for the separation rule.

### Boids Algorithm

The `move_boids` system implements the Boids algorithm. For each boid, it calculates the separation, alignment, and cohesion forces based on its neighbors. These forces are then combined to update the boid's velocity and position.

The neighborhood of a boid is determined by a radius. Different radii are used for each of the three rules. To optimize the process of finding neighbors, a spatial hash is used.

### Configuration

The simulation can be configured by changing the constants in `src/systems.rs`. These include:

-   `NUMBER_OF_BOIDS`: The number of boids in the simulation.
-   `VELOCITY`: The maximum speed of the boids.
-   `BOID_SIZE`: The size of the boids.
-   And various parameters for the Boids rules, such as the radii and strengths of the forces.

## Future Work

-   **Obstacle Avoidance:** Implement a system for boids to avoid obstacles in their environment.
-   **Predator-Prey Behavior:** Introduce a predator boid that hunts the other boids, and have the flock react to the predator.
-   **Interactivity:** Allow the user to interact with the simulation, for example by adding or removing boids, or by changing the simulation parameters in real-time.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
