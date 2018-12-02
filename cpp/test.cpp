#include <string>
#include <iostream>

/*
 * Experimenting with class templates and abstract classes
template 
class Prefilter {
private:
  std::string fileType;
public:
  std::string getFiletype();
  bool        setFiletype(std::string& fileType);
};

template
Prefilter::Prefilter() {
}
*/

class Prefilter {
    std::string fileType;
  public:
    virtual std::string getFileType() = 0;
    virtual bool        setFileType(const std::string& fType) = 0;
};

class PDFPrefilter: public Prefilter {
  private:
    std::string fileType;
  public:
    std::string getFileType() {
      return this->fileType;
    }

    bool setFileType(const std::string& fType) {
      this->fileType = fType;
    }
};

int main() {
  PDFPrefilter pdf;
  std::string temp_filetype("pdf");

  pdf.setFileType(temp_filetype);
  std::cout << pdf.getFileType() << "\n";

  return 0;
}



/* What might go into a Prefilter abstract class ?
 * fType :: String
 * getFtype
 * setFtype()
 * analysis()
 * getAnalysisResults() -- maybe result implement serializable?
 * setFileLocation()
 *
 */
