# Written in Rust with ❤️

## Endpoints

### GET /health
- Returns a simple health check message.

************************************************************************************************

### POST /user/auth/register
- Registers a new user.
- Request Body:
  ```json
  {
    "encrypted_data": "<ENCRYPT THE ABOVE JSON WITH PUB KEY>"
  }
  ```
- Example data:
  ```json
    {
      "name": "John Doe",
      "role": "Software Engineer",
      "email": "john1.doe@example.com",
      "alternate_email": "j.doe@example.com",
      "phone": "+1-555-123-4567",
      "college": "University of Example",
      "graduation_year": 2022,
      "gender":"FEMALE",
      "linkedin": "https://www.linkedin.com/in/johndoe",
      "github": "https://github.com/johndoe"
    }
    ```
- Response:
  ```json
  {
      "message": "User registered successfully",
      "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJqb2huMS5kb2VAZXhhbXBsZS5jb20iLCJleHAiOjE3MjExNTIyMzN9.saGFpXbpm0vdP2jZwwlD4soZqcCd06iEa_J5-78uHLU",
      "user": {
          "alternate_email": "j.doe@example.com",
          "college": "University of Example",
          "email": "john1.doe@example.com",
          "github": "https://github.com/johndoe",
          "graduation_year": null,
          "linkedin": "https://www.linkedin.com/in/johndoe",
          "name": "John Doe",
          "phone": "+1-555-123-4567",
          "role": "Software Engineer",
          "user_id": "5cd6be40-1268-45ad-bd2d-062e33961d23"
      }
  }
  ```
- Note: The `token` is a JWT token that should be used for further requests.Save it in local storage.

************************************************************************************************

### POST /user/auth/authenticate
- Authenticates a user.

- Example data:
  ```json
  {
    "email":"john1.doe@example.com"
  }
  ```
- Request Body:
  ```json
  {
    "encrypted_data": "<ENCRYPT THE ABOVE JSON WITH PUB KEY>"
  }
  ```
- Response:
  ```json
  {
      "message": "User authenticated successfully",
      "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJqb2huMS5kb2VAZXhhbXBsZS5jb20iLCJleHAiOjE3MjExNTI1MDJ9.-9ivceqXG9YhVFmYIO_1Q2qS798kXAy2XKT2je8YOK0",
      "user": {
          "alternate_email": "j.doe@example.com",
          "college": "University of Example",
          "email": "john1.doe@example.com",
          "github": "https://github.com/johndoe",
          "graduation_year": null,
          "linkedin": "https://www.linkedin.com/in/johndoe",
          "name": "John Doe",
          "phone": "+1-555-123-4567",
          "role": "Software Engineer",
          "user_id": "5cd6be40-1268-45ad-bd2d-062e33961d23"
      }
  }
  ```
- Note: The `token` is a JWT token that should be used for further requests.Save it in local storage.

************************************************************************************************

### POST /user/get_details
- Get user details.

- Request Headers:
  ```json
  {
    "Authorization": "Bearer <TOKEN>"
  }
  ```
- Note: Replace `<TOKEN>` with the token received from the `/user/auth/authenticate` endpoint.

- Response:
  ```json
  {
      "message": "User found",
      "user": {
          "alternate_email": "j.doe@example.com",
          "college": "University of Example",
          "email": "john.doe@example.com",
          "github": "https://github.com/johndoe",
          "graduation_year": null,
          "linkedin": "https://www.linkedin.com/in/johndoe",
          "name": "John Doe",
          "phone": "+1-555-123-4567",
          "role": "Software Engineer",
          "user_id": "5677b540-d265-43ac-a386-bfa0bda2d224"
      }
  }
  ```

************************************************************************************************

### POST /check_user
- Check if user exists.

- Request Headers:
  ```json
  {
    "email": "john.doe@example.com"
  }
  ```

- Response:
  ```json
  {
      "message": "User exists"
  }
  ```

************************************************************************************************

### PATCH /user/update-details
- Update user details.

- Request Headers:
  ```json
  {
    "Authorization : Bearer <TOKEN>"
  }
  ```
- Note: Replace `<TOKEN>` with the token received from the `/user/auth/authenticate` endpoint.

- Request Body:
  ```json
  {
    "name": "John Doe",
    "role": "Software Engineer",
    "alternate_email": "jhondoe@gmail.com",
    "phone": "+1-555-123-4567",
    "college": "University of Example",
    "graduation_year": 2022,
    "linkedin": "https://www.linkedin.com/in/johndoe",
    "github": "https://www.github.com/johndoe"
  }
  ```

************************************************************************************************


### POST /insights/create
- Create a new insight.

- Request Body:
  ```json
  {
    "insight_title": "The Future of AI in Healthcare",
    "insight_company": "Tech Health Corp",
    "insight_role": "Lead Data Scientist",
    "insight_tags": ["AI", "Healthcare", "Innovation"],
    "insight_description": "An in-depth analysis of how AI technologies are transforming the healthcare industry, focusing on diagnostics, treatment personalization, and operational efficiency.",
    "insight_picture_urls": [
      "https://example.com/images/insight1.jpg",
      "https://example.com/images/insight2.jpg"
    ],
    "insight_focus_points": [
      "AI in Diagnostics",
      "Personalized Treatment",
      "Operational Efficiency",
      "Ethical Considerations"
    ]
  }
  ```

- Request Headers:
  ```json
  {
    "Authorization": "Bearer <TOKEN>"
  }
  ```
- Note: Replace `<TOKEN>` with the token received from the `/user/auth/authenticate` endpoint.


************************************************************************************************

### POST /insights/get-all
- Get all insights.

************************************************************************************************

### POST /insights/get-recent-insights
- Get recent insights.

in query params:
- `limit`: Number of insights to return. (default: 5)

************************************************************************************************

### POST /insights/delete-insight
- Delete an insight.

in query params:
- `insight_id`: ID of the insight to delete.

- Request Headers:
  ```json
  {
    "Authorization":"Bearer <TOKEN>"
  }
  ```
- Note: Replace `<TOKEN>` with the token received from the `/user/auth/authenticate` endpoint.

************************************************************************************************

### POST /misc/add-college
- Add a new college.

- Request Headers:
  ```json
  {
    "Authorization": "Bearer <TOKEN>"
  }
  ```

- Request Body:
  ```json
  {
    "college_name": "University of Example",
    "college_location": "Example City"
  }
  ```
- Note: Replace `<TOKEN>` with the token received from the `/user/auth/authenticate` endpoint.
************************************************************************************************

  ### POST /misc/add-comapny
  - Add a new company.

  - Request Headers:
    ```json
    {
      "Authorization": "Bearer <TOKEN>"
    }
    ```

  - Request Body:
    ```json
    {
      "company_name": "Comapny Name"
    }
    ```
- Note: Replace `<TOKEN>` with the token received from the `/user/auth/authenticate` endpoint.
************************************************************************************************

### POST /misc/subscribe-newsletter
- Subscribe to the newsletter.

- Request Body:
  ```json
  {
    "email": "johndoe@example.com"
  }
  ```
************************************************************************************************

### POST /misc/get-newsletter-subs
- Get all newsletter subscribers.

- Request Headers:
  ```json
  {
    "Authorization":"Bearer <TOKEN>"
  }
  ```
- Note: Replace `<TOKEN>` with the token received from the `/user/auth/authenticate` endpoint.
************************************************************************************************

### POST /misc/get-tags
- Get all tags.

************************************************************************************************

### POST /misc/get-colleges
- Get all colleges.

************************************************************************************************
