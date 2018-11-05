# rust-web-example

Simple rust web example

## Setup

### Prerequisites

This application uses the [Timeular Public API](https://developers.timeular.com/public-api/), so in order for it to work, you need an account and create developer credentials.

You can either pass these credentials to the application via a `me.secret` file in this folder, which needs to look like this:

```bash
apiKey
apiSecret
```

It's also possible to use the `API_KEY` and `API_SECRET` environment variables.

### Running

Start by using `cargo run`.

This starts a server on port 8080.

You can then start sending requests to e.g.: `GET http://localhost:8080/rest/v1/activities`

Available requests:

* `GET /rest/v1/activities` - returns all activities
* `POST /rest/v1/activities` - creates an activity
* `GET /rest/v1/{activity_id}` - returns an activity with the given id
* `DELETE /rest/v1/{activity_id}` - deletes the activity with the given id
* `PATCH /rest/v1/{activity_id}` - updates the activity with the given id

## Docker

You can also use docker to build and run this application:

```
docker build -t analyzer .
docker run -p 8080:8080 -e API_KEY=apiKey -e API_SECRET=apiSecret analyzer
```
