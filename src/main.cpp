#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <cmark.h>

struct ParsedFile {
    std::string front_matter;
    std::string body;
};

ParsedFile
split_front_matter(const std::string& raw) {
    ParsedFile result;

    const std::string delim= "---\n";
    size_t first = raw.find(delim);

    if (first == std::string::npos) {
        result.body = raw;
        return result;
    }

    size_t second = raw.find(delim, first + delim.size());

    if (second == std::string::npos) {
        std::cerr << "mising closing ---\n";
        result.body = raw;
        return result;
    }

    size_t fm_start = first + delim.size();
    size_t fm_len = second - fm_start;
    result.front_matter = raw.substr(fm_start, fm_len);

    size_t body_start = second + delim.size();
    result.body = raw.substr(body_start);

    return result;
}

int
main(int argc, char** argv) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv [0] << " <file.md>\n";
        return 1;
    }

    std::ifstream file(argv[1]);
    if (!file.is_open()) {
        std::cerr << "cannot open file " << argv[1] << "\n";
        return 1;
    }

    std::stringstream buffer;
    buffer << file.rdbuf();
    std::string raw = buffer.str();
    file.close();

    ParsedFile parsed = split_front_matter(raw);

    if (!parsed.front_matter.empty()) {
        std::cout << "Front Matter\n"
            << parsed.front_matter
            << "End Front Matter\n\n";
    } else {
        std::cout << "(no front matter)\n\n";
    }

    char *html = cmark_markdown_to_html(
            parsed.body.c_str(),
            parsed.body.size(),
            CMARK_OPT_DEFAULT
            );

    if (!html) {
        return 1;
    }

    std::cout << html << "\n";
    free(html);
    return 0;
}
