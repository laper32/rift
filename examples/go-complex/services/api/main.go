package main

import (
	"github.com/gin-gonic/gin"
	"my-workspace/logger"
	"my-workspace/common"
)

func main() {
	r := gin.Default()
	logger.SetupMiddleware(r)

	r.GET("/ping", func(c *gin.Context) {
		c.JSON(200, common.Response{
			ID:      common.NewID(),
			Message: "pong",
		})
	})

	r.Run(":8080")
}
