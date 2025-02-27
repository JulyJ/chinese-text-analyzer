export interface LacOutputItem {
    word: string;
    tag: string;
}

export interface HskAnalysis {
    [k: string]: number;
}

export interface AnalyzerOutput {
    chars_count: number;
    unique_chars_count: number;
    unique_chars: string[];
    words_count: number;
    unique_words_count: number;
    unique_words: string[];
    hsk_analysis: HskAnalysis;
}
