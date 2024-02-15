# Euterpefy API

The Euterpefy API, developed as part of the Euterpefy music recommender project, is a Rust-based service designed to interact directly with the Spotify Web API using API credentials. Focused on fetching data and generating music recommendations, this API simplifies access to Spotify's extensive music catalog without requiring user authentication via OAuth. It's an ideal backend for applications that need to retrieve music data, generate playlists, or explore Spotify's music and podcast offerings.

## Features

- **API Key Authentication**: Utilizes Spotify API credentials for straightforward and secure access to Spotify's data.
- **Music Data Fetching**: Offers endpoints to fetch genres, artists, and tracks from Spotify, providing a rich dataset for music discovery and analysis.
- **Recommendation Generation**: Leverages Spotify's powerful music recommendation algorithms to suggest tracks based on seed genres, artists, or tracks.

## Getting Started

### Prerequisites

- Ensure you have Rust and Cargo installed on your system.
- Obtain Spotify API credentials (Client ID and Client Secret) by registering your application on the Spotify Developer Dashboard.

### Installation and Setup

1. **Clone the Euterpefy API repository:**

   ```sh
   git clone https://github.com/Euterpefy/api.git
   ```

2. **Navigate to the project directory:**

   ```sh
   cd api
   ```

3. **Configure your Spotify API credentials:**

   Set your Spotify Client ID and Client Secret as environment variables or directly within the application configuration.

### Running the API

Execute the following command to start the API server:

```sh
cargo run
```

The API will be available at `http://localhost:8080`, ready to handle requests for music data and recommendations.

If you're connecting from a localhosting Android Studio, use `http://10.0.2.2:8080`

## Usage

The API provides several key endpoints for interacting with Spotify data:

- **Get Seed Genres**: `/api/seeds/genres` - Fetches available seed genres for recommendations.
- **Get Seed Artists**: `/api/seeds/artists` - Retrieves artists based on specified genres.
- **Get Seed Tracks**: `/api/seeds/tracks` - Obtains tracks that can be used as seeds for generating recommendations.
- **Generate Recommendations**: `/api/recommendation-tracks` - Generates a list of recommended tracks based on seed data.

## Documentation

For more detailed information on the endpoints, request parameters, and expected response formats, please refer to the API documentation available in the repository.

## Contributing

Contributions to the Euterpefy API are welcome! If you have ideas for new features, improvements, or bug fixes, feel free to fork the repository, make your changes, and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Spotify for their comprehensive Web API and the wealth of music data it provides.
- The Rust community for the powerful ecosystem that makes projects like this possible.
