```mermaid
gantt
       dateFormat  YY-MM-DD
       title My Schedule before December
	   axisFormat  %m-%d
	   
	   section Reading Papers
	   2 papers per day (50 in total): paper, 19-09-18, 35d
	   
       section Thesis Proposal
       Future task: des3, after des2, 5d
       Future task2              : des4, after des3, 5d

       section Critical tasks
       Completed task in the critical line :crit, done, 19-010-06,24h
       Implement parser and jison          :crit, done, after des1, 2d
       Create tests for parser             :crit, active, 3d
       Future task in critical line    :crit, 5d
       Create tests for renderer           :2d
       Add to mermaid                      :1d

       section Documentation
       Describe gantt syntax               :active, a1, after des1, 3d
       Add gantt diagram to demo page      :after a1  , 20h
       Add another diagram to demo page    :doc1, after a1  , 48h

       section Last section
       Describe gantt syntax               :after doc1, 3d
       Add gantt diagram to demo page      :20h
       Add another diagram to demo page    :48h
```





