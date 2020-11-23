package passwordBasedHashing

import (
	"bytes"
	"crypto/sha256"
	"encoding/base64"
	"fmt"
	"os"
	"strconv"
	"strings"

	"golang.org/x/crypto/pbkdf2"
)

const (
	SaltSize   = 16
	KeySize    = 32
	Iterations = 10000
)

func Hash(input string) string {
	salt := make([]byte, SaltSize)
	key := pbkdf2.Key([]byte(input), salt, Iterations, KeySize, sha256.New)

	saltString := base64.StdEncoding.EncodeToString([]byte(salt))
	keyString := base64.StdEncoding.EncodeToString([]byte(key))

	return fmt.Sprint(Iterations, ".", saltString, ".", keyString) //example output: {10000.a9834ythakg.p1dkfh48bai32}
}

func EqualityCheck(hash string, input string) (Verified bool, NeedsUpgrade bool) {
	parts := strings.Split(hash, ".")
	if len(parts) != 3 {
		//todo
	}
	iterations, err := strconv.Atoi(parts[0])
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}
	salt, err := base64.StdEncoding.DecodeString(parts[1])
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}
	key, err := base64.StdEncoding.DecodeString(parts[2])
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}

	needsUpgrade := iterations == Iterations
	generatedKey := pbkdf2.Key([]byte(input), salt, Iterations, KeySize, sha256.New)
	if bytes.Compare(generatedKey, key) == 0 {
		return true, needsUpgrade
	}
	return false, needsUpgrade
}
