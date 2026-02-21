package common

import "github.com/google/uuid"

type ID string

func NewID() ID {
	return ID(uuid.New().String())
}

type Response struct {
	ID      ID   `json:"id"`
	Message string `json:"message"`
}
