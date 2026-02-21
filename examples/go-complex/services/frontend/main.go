package main

import (
	"github.com/gin-gonic/gin"
	"my-workspace/logger"
)

func main() {
	r := gin.Default()
	logger.SetupMiddleware(r)

	r.GET("/", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"service": "frontend",
			"status": "ok",
		})
	})

	r.Run(":8081")
}
