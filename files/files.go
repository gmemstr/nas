package files

import (
	"encoding/json"
	"fmt"
	"github.com/gmemstr/nas/common"
	"github.com/gorilla/mux"
	"html/template"
	"io"
	"io/ioutil"
	"net/http"
	"os"
	"strings"
)

type Config struct {
	ColdStorage string
	HotStorage  string
}

type Directory struct {
	Path         string
	Files        []os.FileInfo
	Previous     string
	Prefix       string
	SinglePrefix string
}

// Lists out directory using template.
func Listing(tier string) common.Handler {

	return func(rc *common.RouterContext, w http.ResponseWriter, r *http.Request) *common.HTTPError {
		vars := mux.Vars(r)
		id := vars["file"]

		d, err := ioutil.ReadFile("assets/config/config.json")
		if err != nil {
			panic(err)
		}

		var config Config;
		err = json.Unmarshal(d, &config)
		if err != nil {
			panic(err)
		}
		// Default to hot storage
		storage := config.HotStorage
		prefix := "files"
		singleprefix := "file"
		if tier == "cold" {
			storage = config.ColdStorage
			prefix = "archive"
			singleprefix = "archived"
		}
		path := storage

		if id != "" {
			path = path + id
		}
		if err != nil {
			panic(err)
		}

		fileDir, err := ioutil.ReadDir(path)
		path = strings.Replace(path, storage, "", -1)

		// Figure out what our previous location was.
		previous := strings.Split(path, "/")
		previous = previous[:len(previous)-1]
		previousPath := strings.Join(previous, "/")

		directory := Directory{
			Path:         path,
			Files:        fileDir,
			Previous:     previousPath,
			Prefix:       prefix,
			SinglePrefix: singleprefix,
		}

		t, err := template.ParseFiles("assets/web/listing.html")
		if err != nil {
			panic(err)
		}

		t.Execute(w, directory)
		return nil
	}

}

// Lists out directory using template.
func ViewFile(tier string) common.Handler {

	return func(rc *common.RouterContext, w http.ResponseWriter, r *http.Request) *common.HTTPError {
		vars := mux.Vars(r)
		id := vars["file"]

		d, err := ioutil.ReadFile("assets/config/config.json")
		if err != nil {
			panic(err)
		}

		var config Config;
		err = json.Unmarshal(d, &config)
		if err != nil {
			panic(err)
		}
		// Default to hot storage
		storage := config.HotStorage
		if tier == "cold" {
			storage = config.ColdStorage
		}
		path := storage + id
		if err != nil {
			panic(err)
		}

		common.ReadAndServeFile(path, w)
		return nil
	}

}

func UploadFile() common.Handler {

	return func(rc *common.RouterContext, w http.ResponseWriter, r *http.Request) *common.HTTPError {
		d, err := ioutil.ReadFile("assets/config/config.json")
		var config Config;
		err = json.Unmarshal(d, &config)
		if err != nil {
			panic(err)
		}

		err = r.ParseMultipartForm(32 << 20)
		path := strings.Join(r.Form["path"], "")

		// Default to hot storage
		storage := config.HotStorage

		file, handler, err := r.FormFile("file")
		if err != nil {
			fmt.Println(err)
			return nil
		}
		defer file.Close()

		f, err := os.OpenFile(storage+"/"+path+"/"+handler.Filename, os.O_WRONLY|os.O_CREATE, 0666)

		if err != nil {
			panic(err)
		}
		defer f.Close()
		io.Copy(f, file)
		return nil
	}

}
