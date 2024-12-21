#include <iostream>
#include <string>
#include <vector>
#include <memory>
#include <fstream>
#include <regex>
#include <algorithm>
#include <functional>
#include <numeric>

struct File
{
    std::string name;
    size_t size;
    bool is_dir;
    std::vector<std::shared_ptr<File>> files;
    std::weak_ptr<File> parent;

    size_t total_size(size_t current_size = 0) const
    {
        if (!is_dir)
        {
            return size;
        }

        size_t size = current_size;
        for (const auto &f : files)
        {
            size += f->total_size(current_size);
        }

        return size;
    }
};

struct FileSystem
{
    std::shared_ptr<File> root;
    std::weak_ptr<File> cwd;

    FileSystem(const std::string &root_path)
    {
        root = std::make_shared<File>(File{root_path, 0, true, {}, std::shared_ptr<File>(nullptr)});
        cwd = root;
    }

    static FileSystem parse_from_terminal(const std::string &file_path)
    {
        FileSystem fs("/");

        std::ifstream file(file_path);
        if (file)
        {
            std::string line;
            std::getline(file, line); // skip first line "cd /"
            while (std::getline(file, line))
            {
                fs.parse_terminal_line(line);
            }
        }

        return fs;
    }

    void parse_terminal_line(const std::string &line)
    {
        std::regex re_ls_dir("^dir ([a-zA-Z]+)$");
        std::regex re_ls_file("^(\\d+) ([^\\s]+)$");
        std::regex re_cmd_cd("^\\$ cd ([^\\s]+)$");
        std::regex re_cmd_ls("^\\$ ls$");

        std::smatch m;
        if (std::regex_match(line, m, re_ls_dir))
        {
            ls_file(m[1], true, 0);
        }
        else if (std::regex_match(line, m, re_ls_file))
        {
            ls_file(m[2], false, std::atoi(m[1].str().c_str()));
        }
        else if (std::regex_match(line, m, re_cmd_cd))
        {
            cd(m[1]);
        }
        else if (std::regex_match(line, m, re_cmd_ls))
        {
            // nothing to do
        }
    }

    void ls_file(const std::string &name, bool is_dir, size_t size)
    {
        auto dir = cwd.lock();
        if (std::find_if(dir->files.cbegin(), dir->files.cend(),
                         [&](const std::shared_ptr<File> &f) { return f->name == name; }) == dir->files.cend())
        {
            dir->files.push_back(std::make_shared<File>(File{name, size, is_dir, {}, dir}));
            return;
        }
    }

    void cd(const std::string &dir)
    {
        auto pcwd = cwd.lock();
        auto parent = pcwd->parent.lock();
        if (dir == ".." && parent)
        {
            cwd = parent;
            return;
        }

        for (const auto &f : pcwd->files)
        {
            if (f->name == dir)
            {
                cwd = f;
                return;
            }
        }

        pcwd->files.push_back(std::make_shared<File>(File{dir, 0, true, {}, pcwd}));
    }

    void filter_dirs_impl(const std::shared_ptr<File> &current, std::function<bool(const std::shared_ptr<File> &)> &p,
                          std::vector<std::shared_ptr<File>> &res)
    {
        for (const auto &f : current->files)
        {
            if (f->is_dir)
            {
                if (p(f))
                {
                    res.push_back(f);
                }

                filter_dirs_impl(f, p, res);
            }
        }
    }

    std::vector<std::shared_ptr<File>> filter_dirs(std::function<bool(const std::shared_ptr<File> &)> p)
    {
        std::vector<std::shared_ptr<File>> res;
        filter_dirs_impl(root, p, res);
        return res;
    }
};

int main()
{
    auto fs = FileSystem::parse_from_terminal("data/aoc7.txt");

    auto dirs = fs.filter_dirs([](const std::shared_ptr<File> &f) { return f->total_size() < 100000; });
    auto sum = std::accumulate(dirs.cbegin(), dirs.cend(), 0,
                               [](size_t sum, const std::shared_ptr<File> &f) { return sum + f->total_size(); });

    std::cout << "Sum: " << sum << std::endl;

    return 0;
}
