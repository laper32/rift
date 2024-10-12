# Rift.Engine

核心部分，底层数据。

考虑到Deno的架构（启动前还会加载一层JS），所以这里只处理最底层的东西，不涉及到任何Export相关的东西。

其他的东西我们在Runtime做。

N.B.: 我们会用TS减轻写API的心智负担，但一定不要用任何形式的Export！
