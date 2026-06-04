package main

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/golang-jwt/jwt/v5"
	_ "github.com/lib/pq"
	"golang.org/x/crypto/bcrypt"
)

var db *sql.DB

var jwtKey = []byte("my_secret_key_890")

type Claims struct {
	UserID int `json:"user_id"`
	jwt.RegisteredClaims
}

func main() {
	connStr := "postgres://username:password@localhost:5432/postgres?sslmode=disable"

	var err error
	db, err = sql.Open("postgres", connStr)
	if err != nil {
		log.Fatalf("Failed to open connection: %v", err)
	}
	defer db.Close()

	if err = db.Ping(); err != nil {
		log.Fatalf("Database is unreachable: %v", err)
	}
	fmt.Println("Yay! Connected to PostgreSQL")

	r := chi.NewRouter()
	r.Use(middleware.Logger)

	r.Post("/register", registerHandler)
	r.Post("/login", loginHandler)

	fmt.Println("Server starting on port : 8080")
	log.Fatal(http.ListenAndServe(":8080", r))
}

func registerHandler(w http.ResponseWriter, r *http.Request) {
	var input struct {
		Email    string `json:"email"`
		Password string `json:"password"`
	}

	if err := json.NewDecoder(r.Body).Decode(&input); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	hashedPassword, err := bcrypt.GenerateFromPassword([]byte(input.Password), bcrypt.DefaultCost)
	if err != nil {
		http.Error(w, "Failed to secure password", http.StatusInternalServerError)
		return
	}

	query := `INSERT INTO users(email, password_hash) VALUES ($1, $2);`
	_, err = db.Exec(query, input.Email, string(hashedPassword))
	if err != nil {
		http.Error(w, "Could not register (Email might already exists)", http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusCreated)
	w.Write([]byte(`{"messsage": "User registered succesfully!"}`))
}

func loginHandler(w http.ResponseWriter, r *http.Request) {
	var input struct {
		Email    string `json:"email"`
		Password string `json:"password"`
	}

	if err := json.NewDecoder(r.Body).Decode(&input); err != nil {
		http.Error(w, "Invalid request payload", http.StatusBadRequest)
		return
	}

	var dbUserID int
	var dbPasswordHash string
	query := `SELECT id, password_hash FROM users WHERE email = $1 LIMIT 1;`

	err := db.QueryRow(query, input.Email).Scan(&dbUserID, &dbPasswordHash)
	if err == sql.ErrNoRows {
		http.Error(w, "Invalid email or password", http.StatusUnauthorized)
		return
	} else if err != nil {
		http.Error(w, "Server error", http.StatusInternalServerError)
		return
	}

	err = bcrypt.CompareHashAndPassword([]byte(dbPasswordHash), []byte(input.Password))
	if err != nil {
		http.Error(w, "Invalid email or password", http.StatusUnauthorized)
		return
	}

	expirationTime := time.Now().Add(24 * time.Hour)
	claims := &Claims{
		UserID: dbUserID,
		RegisteredClaims: jwt.RegisteredClaims{
			ExpiresAt: jwt.NewNumericDate(expirationTime),
		},
	}

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	tokenString, err := token.SignedString(jwtKey)
	if err != nil {
		http.Error(w, "Could not generate session token", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"token": tokenString})
}
