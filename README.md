# regex_pics_del
可以根据给定的regex表达式将文件夹内的文件分组并按组去重（仅推荐图片使用，在分组后自动删除体积小以及hash相同的文件）
This is a software used to group your files via filename (no ext) and regex, then remove duplicates with each group.

## Example:
### Command:
`D:\\debug\\regex_pics_del.exe "^([\w]{1,})([#\w]{0,})" "D:\\Downloads\\UI_Codex\\"`

### regex:
`^([\w]{{1,}})([#\w]{{0,}})`

### Pics_list:
```
D:\Downloads\UI_Codex\UI_Codex_Scenery_CYCengYanJuYuanZhu#18435810.png
D:\Downloads\UI_Codex\UI_Codex_PhotoDisplayStand_08#2592427.png
D:\Downloads\UI_Codex\UI_Codex_Scenery_CYCengYanJuYuanZhu#2187382.png
D:\Downloads\UI_Codex\UI_Codex_Scenery_CYCengYanJuYuanZhu#2557.png
D:\Downloads\UI_Codex\UI_Codex_Scenery_CYCengYanJuYuanZhu#3239116.png
D:\Downloads\UI_Codex\UI_Codex_PhotoDisplayStand_08.png
D:\Downloads\UI_Codex\UI_Codex_Scenery_CYCengYanJuYuanZhu#67994.png
D:\Downloads\UI_Codex\UI_Codex_Scenery_CYCengYanJuYuanZhu.png
```

### Grouped:
```
---
UI_Codex_PhotoDisplayStand
D:\Downloads\UI_Codex\UI_Codex_PhotoDisplayStand_08.png  7712kb
D:\Downloads\UI_Codex\UI_Codex_PhotoDisplayStand_08#2592427.png  8800kb

---
UI_Codex_Scenery_CYCengYanJuYuanZhu
D:\Downloads\UI_Codex\UI_Codex_Scenery_CYCengYanJuYuanZhu#18435810.png  123kb
D:\Downloads\UI_Codex\UI_Codex_Scenery_CYCengYanJuYuanZhu#2187382.png  456kb
D:\Downloads\UI_Codex\UI_Codex_Scenery_CYCengYanJuYuanZhu#2557.png  678kb
D:\Downloads\UI_Codex\UI_Codex_Scenery_CYCengYanJuYuanZhu#3239116.png 789kb
D:\Downloads\UI_Codex\UI_Codex_Scenery_CYCengYanJuYuanZhu#67994.png 33kb
D:\Downloads\UI_Codex\UI_Codex_Scenery_CYCengYanJuYuanZhu.png 789kb
```
### Result:
```
D:\Downloads\UI_Codex\UI_Codex_PhotoDisplayStand_08#2592427.png
D:\Downloads\UI_Codex\UI_Codex_Scenery_CYCengYanJuYuanZhu#3239116.png
```
