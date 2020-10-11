# ExpRepTem
実験レポート用のテンプレートです．[プレビュー](https://github.com/m12watanabe1a/ExpRepTemp/blob/master/report.pdf)

## セットアップ
以下の``` assets/info.tex ```項目を自分に合わせてセットアップしてください．

```tex
% 名前
\newcommand{\myname}{
    実験 好太郎
}

% 学籍番号
\newcommand{\myid}{
    00-000000
}

% 共同実験者
\newcommand{\mymember}{
    実験 好子，
    実験 好美，
    実験 好夫，
    実験 好二郎，
    実験 好三郎
}

% メールアドレス
\newcommand{\mymail}{
    mymail@address.com
}
```

## 総合タイトルの設定
表紙の総合タイトルは以下から変更できます．

```tex
% Title
\title{総合タイトル} %ここ
\date{\today}
\author{
    \small{\myid} \\
    \myname\thanks{\mymail}
}
```

## レポートを書く

以下を編集してレポートを書きます．

```tex
% 実験レポート1
% ここから

\subtitle{2019/*/*}

\part*{実験1:タイトル}
\section{目的}
\section{実験}
\section{実験結果}
\section{考察}
\section{課題}

% 実験レポート1
% ここまで
```


## 参考文献の設定
ref.bibに参考文献が記載されています．
デフォルトでは以下のようになっており，参考文献の欄にここの内容が反映されます．
ここの内容はBibDeskやJabRef等を利用して参照したい文献の情報を追加してください．
```bib
@Book{Material,
  title = {マテリアル工学実験テキスト},
  year  = {2019},
}
```

## コンパイル
[.latexmk](https://github.com/m12watanabe1a/ExpRepTemp/blob/master/.latexmkrc)ファイルを追加したので以下のコマンドでビルドできます．

```bash
$latexmk report.tex
```

### VS Codeのレシピ
VS Codeで[LaTeX Workshop](https://github.com/James-Yu/LaTeX-Workshop)を使用している場合のsettings.jsonの設定は以下を参考にしてください．
```json
    // LaTeX Setup
    "latex-workshop.latex.recipes": [
        {  
            "name": "latexmk",  
            "tools": [  
              "latexmk"  
            ]  
        },
    ],
    "latex-workshop.latex.tools": [
        {
            "name": "latexmk",
            "command": "latexmk",
        },
    ],
    "latex-workshop.latex.clean.fileTypes": [
        "*.aux", "*.bbl", "*.blg", "*.idx", "*.ind", "*.lof", "*.lot", "*.out", "*.toc", "*.acn", "*.acr", "*.alg", "*.glg", "*.glo", "*.gls", "*.ist", "*.fls", "*.log", "*.fdb_latexmk", "*.synctex.gz",
        "_minted*", "*.nav", "*.snm", "*.vrb",
        "*.run.xml","*.dvi","*.bcf"
    ],
    "latex-workshop.view.pdf.viewer": "tab",
    "latex-workshop.latex.autoClean.run": "onFailed",
```
