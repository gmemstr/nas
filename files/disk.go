package files

import (
	"fmt"
	"io"
	"io/ioutil"
	"os"
	"strings"
)

type DiskProvider struct{
	FileProvider
}

func (dp *DiskProvider) Setup(args map[string]string) bool {
	return true
}

func (dp *DiskProvider) GetDirectory(path string) Directory {
	rp := strings.Join([]string{dp.Location,path}, "/")
	fileDir, err := ioutil.ReadDir(rp)
	if err != nil {
		_ = os.MkdirAll(path, 0644)
	}
	var fileList []FileInfo

	for _, file := range fileDir {
		info := FileInfo{
			IsDirectory: file.IsDir(),
			Name: file.Name(),
		}
		if !info.IsDirectory {
			split := strings.Split(file.Name(), ".")
			info.Extension = split[len(split) - 1]
		}
		fileList = append(fileList, info)
	}

	return Directory{
		Path: rp,
		Files: fileList,
	}
}

func (dp *DiskProvider) ViewFile(path string, w io.Writer) {
	file := strings.Join([]string{dp.Location,path}, "/")
	fileReader, err := os.Open(file)
	if err != nil {
		return
	}
	_, err = io.Copy(w, fileReader)
	if err != nil {
		return
	}
}

func (dp *DiskProvider) SaveFile(file io.Reader, filename string, path string) bool {
	rp := strings.Join([]string{dp.Location,path,filename}, "/")
	f, err := os.OpenFile(rp, os.O_WRONLY|os.O_CREATE, 0644)
	if err != nil {
		fmt.Printf("error creating %v: %v\n", rp, err.Error())
		return false
	}
	defer f.Close()

	_, err = io.Copy(f, file)
	if err != nil {
		fmt.Printf("error writing %v: %v\n", rp, err.Error())
		return false
	}
	return true
}

func (dp *DiskProvider) DetermineType(path string) string {
	rp := strings.Join([]string{dp.Location,path}, "/")
	file, err := os.Stat(rp)
	if err != nil {
		return ""
	}
	if file.IsDir() {
		return "directory"
	}

	return "file"
}

func (dp *DiskProvider) CreateDirectory(path string) bool {
	rp := strings.Join([]string{dp.Location,path}, "/")
	err := os.Mkdir(rp, 0755)
	if err != nil {
		fmt.Printf("Error creating directory %v: %v\n", rp, err.Error())
		return false
	}
	return true
}

func (dp *DiskProvider) Delete(path string) bool {
	rp := strings.Join([]string{dp.Location,path}, "/")
	err := os.RemoveAll(rp)
	if err != nil {
		fmt.Printf("Error removing %v: %v\n", path, err.Error())
		return false
	}
	return true
}